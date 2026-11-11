use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{
    app::routes::AuthRoute,
    features::auth::lib::auth::AuthContext,
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Search)]
pub fn search() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");

    if !auth.is_authenticated() {
        return html! { <Redirect<AuthRoute> to={AuthRoute::SignIn} /> };
    }

    html! {
        <Wrapper classes="page search">
            <Head classes="page__head search__head" />
            <Body classes="page__body search__body">
                {"search"}
            </Body>
        </Wrapper>
    }
}
