use crate::{app::AuthRoute, features::auth::AuthContext};
use yew::prelude::*;
use yew_router::components::Redirect;

#[derive(Properties, PartialEq)]
pub struct RequireAuthProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RequireAuth)]
pub fn require_auth(props: &RequireAuthProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if !auth.is_ready {
        return html! { <p class="auth-guard__loading" role="status">{"Checking your session..."}</p> };
    }

    if !auth.is_authenticated() {
        return html! { <Redirect<AuthRoute> to={AuthRoute::SignIn} /> };
    }

    html! { <>{ for props.children.iter() }</> }
}
