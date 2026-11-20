use yew::prelude::*;

use crate::{
    app::{AuthRoute, Route},
    features::auth::AuthContext,
    shared::{body::Body, intro::Intro, link::Link, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(About)]
pub fn about() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let is_authenticated = auth.is_authenticated();

    html! {
        <Wrapper classes="page">
            <Head classes="page__head" />
            <Body classes="page__body">
                <Intro
                    classes="page__intro"
                    subtitle="A home for ideas."
                    title="POSTIX"
                    description="Share quick thoughts, publish thoughtful stories, and discover perspectives worth your time"
                >
                    if is_authenticated {
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    } else {
                        <Link<AuthRoute> to={AuthRoute::SignUp}>{"Join"}</Link<AuthRoute>>
                    }
                </Intro>
            </Body>
        </Wrapper>
    }
}
