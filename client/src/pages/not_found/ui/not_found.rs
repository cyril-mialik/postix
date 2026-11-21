use yew::prelude::*;

use crate::{
    shared::{body::Body, intro::Intro, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Wrapper classes="page">
            <Head classes="page__head" />
            <Body classes="page__body">
                <Intro
                    classes="page__intro"
                    subtitle="No such page"
                    title="POSTIX"
                    description="The page you are looking for does not exist. Please check the URL and try again."
                />
            </Body>
        </Wrapper>
    }
}
