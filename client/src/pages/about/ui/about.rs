use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::AuthRoute,
    shared::{body::Body, intro::Intro, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <Wrapper classes="page about">
            <Head classes="page__head about__head" />
            <Body classes="page__body about__body">
                <Intro
                    subtitle="POSTIX"
                    title="A home for ideas."
                    description="Share quick thoughts, publish thoughtful stories, and discover perspectives worth your time."
                >
                        <Link<AuthRoute> to={AuthRoute::SignUp} classes="about__primary">{"Join Postix"}</Link<AuthRoute>>
                        <Link<AuthRoute> to={AuthRoute::SignIn} classes="about__secondary">{"Sign in"}</Link<AuthRoute>>
                </Intro>
            </Body>
        </Wrapper>
    }
}
