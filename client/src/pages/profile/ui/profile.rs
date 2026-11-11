use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{
    app::routes::AuthRoute,
    features::auth::lib::auth::AuthContext,
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if !auth.is_authenticated() {
        return html! { <Redirect<AuthRoute> to={AuthRoute::SignIn} /> };
    }

    html! {
        <Wrapper classes="page profile">
            <Head classes="page__head profile__head" />
            <Body classes="page__body profile__body">
                {"profile"}
            </Body>
        </Wrapper>
    }
}
