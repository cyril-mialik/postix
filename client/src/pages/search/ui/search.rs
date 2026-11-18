use yew::prelude::*;

use crate::{
    shared::{
        body::Body,
        input::{Input, InputType},
        wrapper::Wrapper,
    },
    widgets::{head::Head, post_feed::PostFeed},
};

#[function_component(Search)]
pub fn search() -> Html {
    let query = use_state(String::new);
    let on_query = {
        let query = query.clone();
        Callback::from(move |value: String| query.set(value))
    };

    html! {
        <Wrapper classes="page search">
            <Head classes="page__head search__head" />
            <Body classes="page__body search__body">
                <div class="search__content">
                    <header class="search__intro blueprint-block">
                        <p class="search__eyebrow">{"DISCOVER"}</p>
                        <h1>{"Find a story"}</h1>
                        <p>{"Search recent stories by title, text, or tag."}</p>
                    </header>
                    <Input
                        id="story-search"
                        name="q"
                        label="Search"
                        input_type={InputType::Text}
                        value={(*query).clone()}
                        placeholder="Try “Rust” or “writing”"
                        on_change={on_query}
                        classes="search__input blueprint-block"
                    />
                    <PostFeed query={(*query).clone()} />
                </div>
            </Body>
        </Wrapper>
    }
}
