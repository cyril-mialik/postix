use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::post::PostResponse;

use crate::{
    entities::post::{content_text, post_api},
    features::auth::AuthContext,
    shared::confirm_dialog::ConfirmDialog,
};

#[derive(Properties, PartialEq)]
pub struct EditPostFormProps {
    pub post: PostResponse,
    pub on_saved: Callback<PostResponse>,
    pub on_cancel: Callback<()>,
}

#[function_component(EditPostForm)]
pub fn edit_post_form(props: &EditPostFormProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let title = use_state(|| props.post.title.clone());
    let text = use_state(|| content_text(&props.post));
    let tags = use_state(|| {
        props
            .post
            .tags
            .iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    });
    let error = use_state(|| None::<String>);
    let saving = use_state(|| false);
    let confirming = use_state(|| false);

    let on_submit = {
        let confirming = confirming.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            confirming.set(true);
        })
    };
    let on_confirm = {
        let title = title.clone();
        let text = text.clone();
        let tags = tags.clone();
        let error = error.clone();
        let saving = saving.clone();
        let confirming = confirming.clone();
        let auth = auth.clone();
        let post_id = props.post.id.to_string();
        let on_saved = props.on_saved.clone();
        Callback::from(move |_| {
            if *saving {
                return;
            }
            error.set(None);
            saving.set(true);
            let title_value = (*title).clone();
            let text_value = (*text).clone();
            let tags_value = (*tags).clone();
            let error = error.clone();
            let saving = saving.clone();
            let confirming = confirming.clone();
            let auth = auth.clone();
            let post_id = post_id.clone();
            let on_saved = on_saved.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let post_id = post_id.clone();
                        let title = title_value.clone();
                        let text = text_value.clone();
                        let tags = tags_value.clone();
                        async move {
                            post_api::update_post(&token, &post_id, &title, &text, &tags).await
                        }
                    })
                    .await;
                match result {
                    Ok(post) => {
                        confirming.set(false);
                        on_saved.emit(post);
                    }
                    Err(message) => {
                        error.set(Some(message.to_string()));
                        confirming.set(false);
                    }
                }
                saving.set(false);
            });
        })
    };
    let on_cancel_confirmation = {
        let confirming = confirming.clone();
        Callback::from(move |_| confirming.set(false))
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
    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <>
        <form class="create-post blueprint-block" onsubmit={on_submit}>
            <label class="create-post__label" for="edit-post-title">{"Title"}</label>
            <input
                id="edit-post-title"
                class="create-post__input"
                type="text"
                value={(*title).clone()}
                minlength="3"
                maxlength="256"
                required=true
                disabled={*saving}
                oninput={on_title}
            />
            <label class="create-post__label" for="edit-post-text">{"Your story"}</label>
            <textarea
                id="edit-post-text"
                class="create-post__textarea"
                value={(*text).clone()}
                maxlength="30000"
                rows="5"
                required=true
                disabled={*saving}
                oninput={on_text}
            />
            <label class="create-post__label" for="edit-post-tags">{"Tags"}</label>
            <input
                id="edit-post-tags"
                class="create-post__input"
                type="text"
                value={(*tags).clone()}
                placeholder="rust, writing (comma separated)"
                disabled={*saving}
                oninput={on_tags}
            />
            if let Some(message) = &*error {
                <p class="create-post__error blueprint-block blueprint-block--error" role="alert">{message}</p>
            }
            <div class="create-post__buttons">
                <button class="action--red" type="button" disabled={*saving} onclick={on_cancel}>
                    {"Cancel"}
                </button>
                <button class="action--green" type="submit" disabled={*saving}>
                    {if *saving { "Saving..." } else { "Save changes" }}
                </button>
            </div>
        </form>
        if *confirming {
            <ConfirmDialog
                title="Save changes?"
                message="The edited story will replace its current version. A snapshot of the previous version will be kept."
                confirm_label="Save changes"
                confirm_classes={classes!("action--green")}
                busy={*saving}
                on_confirm={on_confirm}
                on_cancel={on_cancel_confirmation}
            />
        }
        </>
    }
}
