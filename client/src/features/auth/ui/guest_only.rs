use crate::{app::routes::Route, features::auth::model::auth_context::AuthContext};
use yew::prelude::*;
use yew_router::components::Redirect;

#[derive(Properties, PartialEq)]
pub struct GuestOnlyProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GuestOnly)]
pub fn guest_only(props: &GuestOnlyProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if auth.is_authenticated() {
        return html! { <Redirect<Route> to={Route::Home} /> };
    }

    html! { <>{ for props.children.iter() }</> }
}
