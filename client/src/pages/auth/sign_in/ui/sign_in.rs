use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{
    app::routes::Route,
    features::auth::lib::auth::AuthContext,
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if auth.is_authenticated() {
        return html! { <Redirect<Route> to={Route::Home} /> };
    }

    html! {
        <Wrapper classes="page sign-in">
            <Head classes="page__head sign-in__head" />
            <Body classes="page__body sign-in__body">
                {"Sign In page"}
            </Body>
        </Wrapper>
    }
}
