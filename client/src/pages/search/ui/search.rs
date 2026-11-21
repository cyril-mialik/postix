use yew::prelude::*;

use crate::{
    entities::post::SearchField,
    shared::{
        body::Body,
        input::{Input, InputType},
        intro::Intro,
        wrapper::Wrapper,
    },
    widgets::{head::Head, post_feed::PostFeed},
};

#[function_component(Search)]
pub fn search() -> Html {
    let query = use_state(String::new);
    let active_query = use_state(String::new);
    let field = use_state(|| "all".to_owned());
    let period = use_state(|| "all".to_owned());
    let sort = use_state(|| "newest".to_owned());
    let on_query = {
        let query = query.clone();
        Callback::from(move |value: String| query.set(value))
    };
    let on_submit = {
        let query = query.clone();
        let active_query = active_query.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            active_query.set((*query).clone());
        })
    };
    let on_field = {
        let field = field.clone();
        Callback::from(move |event: Event| {
            let select: web_sys::HtmlSelectElement = event.target_unchecked_into();
            field.set(select.value());
        })
    };
    let on_period = {
        let period = period.clone();
        Callback::from(move |event: Event| {
            let select: web_sys::HtmlSelectElement = event.target_unchecked_into();
            period.set(select.value());
        })
    };
    let on_sort = {
        let sort = sort.clone();
        Callback::from(move |event: Event| {
            let select: web_sys::HtmlSelectElement = event.target_unchecked_into();
            sort.set(select.value());
        })
    };
    let search_field = match field.as_str() {
        "title" => SearchField::Title,
        "content" => SearchField::Content,
        _ => SearchField::All,
    };

    html! {
        <Wrapper classes="page">
            <Head classes="page__head" />
            <Body classes="page__body">
                <Intro
                    subtitle="DISCOVER"
                    title="Find a story"
                    description="Search stories by title, text, tag, or author."
                />
                <form class="search__form blueprint-block" onsubmit={on_submit}>
                    <Input
                        id="story-search"
                        name="q"
                        label="Search"
                        input_type={InputType::Text}
                        value={(*query).clone()}
                        placeholder="Try “Rust”, “#writing”, or “@author”"
                        on_change={on_query}
                        classes="search__input"
                    />
                    <div class="search__filters">
                        <label for="search-field">{"In"}</label>
                        <select id="search-field" value={(*field).clone()} onchange={on_field}>
                            <option value="all">{"Title, content, and tags"}</option>
                            <option value="title">{"Title"}</option>
                            <option value="content">{"Content"}</option>
                        </select>
                        <label for="search-period">{"When"}</label>
                        <select id="search-period" value={(*period).clone()} onchange={on_period}>
                            <option value="all">{"Any time"}</option>
                            <option value="day">{"Past day"}</option>
                            <option value="week">{"Past week"}</option>
                            <option value="month">{"Past month"}</option>
                        </select>
                        <label for="search-sort">{"Sort"}</label>
                        <select id="search-sort" value={(*sort).clone()} onchange={on_sort}>
                            <option value="newest">{"Newest"}</option>
                            <option value="popular">{"Popular"}</option>
                        </select>
                    </div>
                    <button class="search__submit" type="submit">{"Find"}</button>
                </form>
                <PostFeed
                    query={(*active_query).clone()}
                    search_field={search_field}
                    period={(*period).clone()}
                    sort={(*sort).clone()}
                />
            </Body>
        </Wrapper>
    }
}
