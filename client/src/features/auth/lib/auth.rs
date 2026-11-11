use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub token: Option<String>,
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
    let token = use_state(|| {
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("access_token").ok().flatten())
    });

    let context = AuthContext {
        token: (*token).clone(),
    };

    html! {
        <ContextProvider<AuthContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<AuthContext>>
    }
}
