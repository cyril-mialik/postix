use yew::prelude::*;
use yew_router::prelude::*;

use postix_core::post::PostPublicationStatus;

use crate::{
    app::Route,
    features::create_post::CreatePostForm,
    shared::{body::Body, intro::Intro, wrapper::Wrapper},
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
                    <Intro
                        subtitle="NEW STORY"
                        title="Write for Postix"
                        description="Share a quick thought or take the time to tell the whole story."
                    />
                    <CreatePostForm {on_created} />
                </div>
            </Body>
        </Wrapper>
    }
}
