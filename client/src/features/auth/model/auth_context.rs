use std::{future::Future, pin::Pin, rc::Rc};

use super::super::{
    api::auth_api::{self, RefreshError},
    lib::storage,
};
use crate::shared::ApiError;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

type RefreshFuture = Pin<Box<dyn Future<Output = Result<String, RefreshError>>>>;

#[derive(Clone)]
pub struct AuthContext {
    pub token: Option<String>,
    pub is_ready: bool,
    pub login: Callback<String>,
    pub logout: Callback<()>,
    refresh_access_token: Rc<dyn Fn() -> RefreshFuture>,
}

impl PartialEq for AuthContext {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token && self.is_ready == other.is_ready
    }
}

impl AuthContext {
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub async fn with_refresh<T, F, Fut>(&self, request: F) -> Result<T, ApiError>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Result<T, ApiError>>,
    {
        let Some(token) = self.token.clone() else {
            return Err(ApiError::Unauthorized);
        };

        match request(token).await {
            Err(ApiError::Unauthorized) => match (self.refresh_access_token)().await {
                Ok(token) => match request(token).await {
                    Err(ApiError::Unauthorized) => {
                        self.logout.emit(());
                        Err(ApiError::Unauthorized)
                    }
                    result => result,
                },
                Err(RefreshError::InvalidSession) => {
                    self.logout.emit(());
                    Err(ApiError::Unauthorized)
                }
                Err(RefreshError::Unavailable(message)) => Err(ApiError::Message(message)),
            },
            result => result,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| None::<String>);
    let is_ready = use_state(|| false);

    let login = {
        let token = token.clone();
        let is_ready = is_ready.clone();
        Callback::from(move |new_token: String| {
            storage::save_token(&new_token);
            token.set(Some(new_token));
            is_ready.set(true);
        })
    };

    let logout = {
        let token = token.clone();
        let is_ready = is_ready.clone();
        Callback::from(move |_| {
            storage::remove_token();
            token.set(None);
            is_ready.set(true);
        })
    };

    {
        let token = token.clone();
        let is_ready = is_ready.clone();
        use_effect_with((), move |_| {
            let cached_token = storage::get_token();
            spawn_local(async move {
                match auth_api::refresh_access_token().await {
                    Ok(access_token) => {
                        storage::save_token(&access_token);
                        token.set(Some(access_token));
                    }
                    Err(RefreshError::InvalidSession) => {
                        storage::remove_token();
                        token.set(None);
                    }
                    Err(RefreshError::Unavailable(_)) => token.set(cached_token),
                }
                is_ready.set(true);
            });
            || ()
        });
    }

    let refresh_access_token = {
        let token = token.clone();
        Rc::new(move || {
            let token = token.clone();
            Box::pin(async move {
                match auth_api::refresh_access_token().await {
                    Ok(access_token) => {
                        storage::save_token(&access_token);
                        token.set(Some(access_token.clone()));
                        Ok(access_token)
                    }
                    Err(RefreshError::InvalidSession) => {
                        storage::remove_token();
                        token.set(None);
                        Err(RefreshError::InvalidSession)
                    }
                    Err(error @ RefreshError::Unavailable(_)) => Err(error),
                }
            }) as RefreshFuture
        })
    };

    let context = AuthContext {
        token: (*token).clone(),
        is_ready: *is_ready,
        login,
        logout,
        refresh_access_token,
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::Cell,
        task::{Context, Poll, Waker},
    };

    use super::*;

    fn run_ready<T>(future: impl Future<Output = T>) -> T {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = Box::pin(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly yielded"),
        }
    }

    fn context(
        refresh_result: Result<String, RefreshError>,
        logout_called: Rc<Cell<bool>>,
    ) -> AuthContext {
        AuthContext {
            token: Some("expired".to_owned()),
            is_ready: true,
            login: Callback::from(|_: String| {}),
            logout: Callback::from(move |_: ()| logout_called.set(true)),
            refresh_access_token: Rc::new(move || {
                let result = refresh_result.clone();
                Box::pin(async move { result }) as RefreshFuture
            }),
        }
    }

    #[test]
    fn expired_access_tokens_are_refreshed_and_the_request_is_retried() {
        let logout_called = Rc::new(Cell::new(false));
        let auth = context(Ok("fresh".to_owned()), logout_called.clone());
        let attempts = Rc::new(Cell::new(0));

        let result = run_ready(auth.with_refresh({
            let attempts = attempts.clone();
            move |token| {
                attempts.set(attempts.get() + 1);
                async move {
                    if token == "expired" {
                        Err(ApiError::Unauthorized)
                    } else {
                        Ok(token)
                    }
                }
            }
        }));

        assert_eq!(result, Ok("fresh".to_owned()));
        assert_eq!(attempts.get(), 2);
        assert!(!logout_called.get());
    }

    #[test]
    fn invalid_refresh_sessions_clear_authentication() {
        let logout_called = Rc::new(Cell::new(false));
        let auth = context(Err(RefreshError::InvalidSession), logout_called.clone());

        let result =
            run_ready(auth.with_refresh(|_| async { Err::<(), _>(ApiError::Unauthorized) }));

        assert_eq!(result, Err(ApiError::Unauthorized));
        assert!(logout_called.get());
    }

    #[test]
    fn a_rejected_refreshed_token_also_clears_authentication() {
        let logout_called = Rc::new(Cell::new(false));
        let auth = context(Ok("fresh".to_owned()), logout_called.clone());

        let result =
            run_ready(auth.with_refresh(|_| async { Err::<(), _>(ApiError::Unauthorized) }));

        assert_eq!(result, Err(ApiError::Unauthorized));
        assert!(logout_called.get());
    }
}
