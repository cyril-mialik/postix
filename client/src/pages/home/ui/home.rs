use yew::prelude::*;

use crate::{
    shared::{body::Body, intro::Intro, wrapper::Wrapper},
    widgets::{head::Head, post_feed::PostFeed},
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Wrapper classes="page">
            <Head classes="page__head" />
            <Body classes="page__body">
                <Intro
                    classes="page__intro"
                    subtitle="POSTIX JOURNAL"
                    title="Ideas worth sharing."
                    description="Short thoughts and long-form stories, all in one place."
                />

                <PostFeed />
            </Body>
        </Wrapper>
    }
}
