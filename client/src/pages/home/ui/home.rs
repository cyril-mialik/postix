use yew::prelude::*;

use crate::{
    shared::{body::Body, wrapper::Wrapper},
    widgets::{head::Head, post_feed::PostFeed},
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Wrapper classes="page home">
            <Head classes="page__head home__head" />
            <Body classes="page__body home__body">
                <div class="home__content">
                    <header class="home__intro blueprint-block">
                        <p class="home__eyebrow">{"POSTIX JOURNAL"}</p>
                        <h1>{"Ideas worth sharing."}</h1>
                        <p>{"Short thoughts and long-form stories, all in one place."}</p>
                    </header>
                    <PostFeed />
                </div>
            </Body>
        </Wrapper>
    }
}
