use yew::prelude::*;

use crate::{
    features::main_menu::MainMenu,
    shared::{body::Body, intro::Intro, wrapper::Wrapper},
};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <Wrapper classes="page">
            <MainMenu classes="page__head" />
            <Body classes="page__body">
                <Intro
                    classes="page__intro"
                    subtitle="A home for ideas."
                    title="POSTIX"
                    description="Share quick thoughts, publish thoughtful stories, and discover perspectives worth your time"
                />
            </Body>
        </Wrapper>
    }
}
