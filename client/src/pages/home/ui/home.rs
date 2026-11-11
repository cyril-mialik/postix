use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{
    app::routes::AuthRoute,
    features::auth::lib::auth::AuthContext,
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Home)]
pub fn home() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if !auth.is_authenticated() {
        return html! { <Redirect<AuthRoute> to={AuthRoute::SignIn} /> };
    }

    html! {
        <Wrapper classes="page home">
            <Head classes="page__head home__head" />
            <Body classes="page__body home__body">{"home"}</Body>
        </Wrapper>
    }
}
