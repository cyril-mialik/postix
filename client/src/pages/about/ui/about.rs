use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::AuthRoute,
    shared::{block::Block, body::Body, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <Wrapper classes="page about">
            <Head classes="page__head about__head" />
            <Body classes="page__body about__body">
                <Block classes="about__content">
                    <p class="about__eyebrow">{"POSTIX"}</p>
                    <h1>{"A home for ideas."}</h1>
                    <p>{"Share quick thoughts, publish thoughtful stories, and discover perspectives worth your time."}</p>
                    <div class="about__actions">
                        <Link<AuthRoute> to={AuthRoute::SignUp} classes="about__primary">{"Join Postix"}</Link<AuthRoute>>
                        <Link<AuthRoute> to={AuthRoute::SignIn} classes="about__secondary">{"Sign in"}</Link<AuthRoute>>
                    </div>
                </Block>
            </Body>
        </Wrapper>
    }
}
