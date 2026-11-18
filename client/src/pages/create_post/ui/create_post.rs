use yew::prelude::*;
use yew_router::prelude::*;

use postix_core::post::PostPublicationStatus;

use crate::{
    app::Route,
    features::create_post::CreatePostForm,
    shared::{body::Body, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(CreatePostPage)]
pub fn create_post_page() -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let on_created = Callback::from(move |status| {
        if status == PostPublicationStatus::Draft {
            navigator.push(&Route::Profile);
        } else {
            navigator.push(&Route::Home);
        }
    });

    html! {
        <Wrapper classes="page create-post-page">
            <Head classes="page__head create-post-page__head" />
            <Body classes="page__body create-post-page__body">
                <div class="create-post-page__content">
                    <header class="create-post-page__intro blueprint-block">
                        <p class="create-post-page__eyebrow">{"NEW STORY"}</p>
                        <h1>{"Write for Postix"}</h1>
                        <p>{"Share a quick thought or take the time to tell the whole story."}</p>
                    </header>
                    <CreatePostForm {on_created} />
                </div>
            </Body>
        </Wrapper>
    }
}
