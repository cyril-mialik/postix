use crate::features::auth::lib::storage;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
    pub login: Callback<String>,
    pub logout: Callback<()>,
}

impl AuthContext {
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| storage::get_token());

    let login = {
        let token = token.clone();
        Callback::from(move |new_token: String| {
            storage::save_token(&new_token);
            token.set(Some(new_token));
        })
    };

    let logout = {
        let token = token.clone();
        Callback::from(move |_| {
            storage::remove_token();
            token.set(None);
        })
    };

    let context = AuthContext {
        token: (*token).clone(),
        login,
        logout,
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}
