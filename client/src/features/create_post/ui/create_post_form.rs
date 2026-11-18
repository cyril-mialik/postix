use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::post::PostPublicationStatus;

use crate::{
    entities::post::post_api, features::auth::AuthContext, shared::confirm_dialog::ConfirmDialog,
};

#[derive(Properties, PartialEq)]
pub struct CreatePostFormProps {
    pub on_created: Callback<PostPublicationStatus>,
}

#[function_component(CreatePostForm)]
pub fn create_post_form(props: &CreatePostFormProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let title = use_state(String::new);
    let text = use_state(String::new);
    let tags = use_state(String::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);
    let confirming = use_state(|| false);
    let pending_status = use_state(|| None::<PostPublicationStatus>);

    let on_submit = {
        let confirming = confirming.clone();
        let pending_status = pending_status.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            pending_status.set(Some(PostPublicationStatus::Published));
            confirming.set(true);
        })
    };

    let on_save_draft = {
        let confirming = confirming.clone();
        let pending_status = pending_status.clone();
        Callback::from(move |_| {
            pending_status.set(Some(PostPublicationStatus::Draft));
            confirming.set(true);
        })
    };

    let on_cancel = {
        let confirming = confirming.clone();
        let pending_status = pending_status.clone();
        Callback::from(move |_| {
            pending_status.set(None);
            confirming.set(false);
        })
    };

    let on_confirm = {
        let title = title.clone();
        let text = text.clone();
        let tags = tags.clone();
        let error = error.clone();
        let loading = loading.clone();
        let confirming = confirming.clone();
        let pending_status = pending_status.clone();
        let auth = auth.clone();
        let on_created = props.on_created.clone();

        Callback::from(move |_| {
            if *loading {
                return;
            }
            let Some(status) = *pending_status else {
                return;
            };

            loading.set(true);
            error.set(None);
            let title_value = (*title).clone();
            let text_value = (*text).clone();
            let tags_value = (*tags).clone();
            let error = error.clone();
            let loading = loading.clone();
            let title = title.clone();
            let text = text.clone();
            let tags = tags.clone();
            let confirming = confirming.clone();
            let pending_status = pending_status.clone();
            let auth = auth.clone();
            let on_created = on_created.clone();

            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let title = title_value.clone();
                        let text = text_value.clone();
                        let tags = tags_value.clone();
                        async move {
                            post_api::create_post(&token, &title, &text, &tags, status).await
                        }
                    })
                    .await;
                match result {
                    Ok(_) => {
                        title.set(String::new());
                        text.set(String::new());
                        tags.set(String::new());
                        confirming.set(false);
                        pending_status.set(None);
                        on_created.emit(status);
                    }
                    Err(message) => {
                        error.set(Some(message.to_string()));
                        confirming.set(false);
                        pending_status.set(None);
                    }
                }
                loading.set(false);
            });
        })
    };

    let on_title = {
        let title = title.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            title.set(input.value());
        })
    };
    let on_text = {
        let text = text.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
            text.set(input.value());
        })
    };
    let on_tags = {
        let tags = tags.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            tags.set(input.value());
        })
    };

    html! {
        <>
            <form class="create-post blueprint-block" onsubmit={on_submit}>
                <label class="create-post__label" for="post-title">{"Title"}</label>
                <input
                    id="post-title"
                    class="create-post__input"
                    type="text"
                    value={(*title).clone()}
                    minlength="3"
                    maxlength="256"
                    placeholder="Give your story a title"
                    required=true
                    disabled={*loading}
                    oninput={on_title}
                />
                <label class="create-post__label" for="post-text">{"Your story"}</label>
                <textarea
                    id="post-text"
                    class="create-post__textarea"
                    value={(*text).clone()}
                    rows="5"
                    placeholder="What would you like to share?"
                    required=true
                    disabled={*loading}
                    oninput={on_text}
                />
                <label class="create-post__label" for="post-tags">{"Tags"}</label>
                <input
                    id="post-tags"
                    class="create-post__input"
                    type="text"
                    value={(*tags).clone()}
                    placeholder="rust, writing (comma separated)"
                    disabled={*loading}
                    oninput={on_tags}
                />
                if let Some(message) = &*error {
                    <p class="create-post__error blueprint-block blueprint-block--error" role="alert">{message}</p>
                }
                <div class="create-post__actions">
                    <div class="create-post__buttons">
                        <button
                            class="create-post__draft"
                            type="button"
                            disabled={*loading}
                            onclick={on_save_draft}
                        >
                            {"Save as draft"}
                        </button>
                        <button class="create-post__submit action--green" type="submit" disabled={*loading}>
                            {"Publish story"}
                        </button>
                    </div>
                    <span class="create-post__hint">{"Choose whether to publish your story or save it as a draft."}</span>
                </div>
            </form>
            if *confirming {
                if let Some(status) = *pending_status {
                    <ConfirmDialog
                        title={if status == PostPublicationStatus::Draft { "Save draft?" } else { "Publish story?" }}
                        message={if status == PostPublicationStatus::Draft {
                            "This story will be saved in the Draft tab on your profile."
                        } else {
                            "This story will be visible in the public feed."
                        }}
                        confirm_label={if status == PostPublicationStatus::Draft { "Save draft" } else { "Publish" }}
                        confirm_classes={classes!(
                            if status == PostPublicationStatus::Published { "action--green" } else { "" }
                        )}
                        busy={*loading}
                        on_confirm={on_confirm}
                        on_cancel={on_cancel}
                    />
                }
            }
        </>
    }
}
