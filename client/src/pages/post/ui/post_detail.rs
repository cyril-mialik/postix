use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use postix_core::{post::PostResponse, user::UserResponse};

use crate::{
    app::Route,
    entities::post::{content_text, post_api},
    entities::user::user_api,
    features::{auth::AuthContext, edit_post::EditPostForm, rate_post::PostRating},
    shared::{body::Body, confirm_dialog::ConfirmDialog, wrapper::Wrapper},
    widgets::{head::Head, review_section::ReviewSection},
};

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
    pub id: String,
    #[prop_or_default]
    pub editing_mode: bool,
}

#[function_component(PostDetail)]
pub fn post_detail(props: &PostDetailProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().expect("Navigator not found");
    let post = use_state(|| None::<PostResponse>);
    let current_user = use_state(|| None::<UserResponse>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let editing_post = use_state(|| props.editing_mode);
    let editing_mode = props.editing_mode;
    let confirming_post_delete = use_state(|| false);
    let deleting_post = use_state(|| false);

    {
        let post = post.clone();
        let current_user = current_user.clone();
        let error = error.clone();
        let loading = loading.clone();
        let editing_post = editing_post.clone();
        let auth = auth.clone();
        let id = props.id.clone();
        use_effect_with((auth.is_ready, id.clone()), move |_| {
            post.set(None);
            current_user.set(None);
            error.set(None);
            loading.set(true);
            editing_post.set(editing_mode);
            if auth.is_ready {
                spawn_local(async move {
                    let result = auth
                        .with_refresh(|token| {
                            let id = id.clone();
                            async move { post_api::get_post(&token, &id).await }
                        })
                        .await;
                    match result {
                        Ok(result) => {
                            post.set(Some(result));
                            let profile_result = auth
                                .with_refresh(|token| async move { user_api::get_me(&token).await })
                                .await;
                            match profile_result {
                                Ok(user) => current_user.set(Some(user)),
                                Err(message) => error.set(Some(message.to_string())),
                            }
                        }
                        Err(message) => error.set(Some(message.to_string())),
                    }
                    loading.set(false);
                });
            } else {
                loading.set(false);
            }
            || ()
        });
    }

    let on_edit_post = {
        let navigator = navigator.clone();
        let id = props.id.clone();
        Callback::from(move |_| navigator.push(&Route::EditPost { id: id.clone() }))
    };
    let on_cancel_post_edit = {
        let navigator = navigator.clone();
        let id = props.id.clone();
        Callback::from(move |_| navigator.push(&Route::Post { id: id.clone() }))
    };
    let on_post_saved = {
        let post = post.clone();
        let navigator = navigator.clone();
        let id = props.id.clone();
        Callback::from(move |updated: PostResponse| {
            post.set(Some(updated));
            navigator.push(&Route::Post { id: id.clone() });
        })
    };
    let on_open_post_delete = {
        let confirming_post_delete = confirming_post_delete.clone();
        Callback::from(move |_| confirming_post_delete.set(true))
    };
    let on_cancel_post_delete = {
        let confirming_post_delete = confirming_post_delete.clone();
        Callback::from(move |_| confirming_post_delete.set(false))
    };
    let on_confirm_post_delete = {
        let auth = auth.clone();
        let post_id = props.id.clone();
        let deleting_post = deleting_post.clone();
        let confirming_post_delete = confirming_post_delete.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |_| {
            if *deleting_post {
                return;
            }
            deleting_post.set(true);
            error.set(None);
            let auth = auth.clone();
            let post_id = post_id.clone();
            let deleting_post = deleting_post.clone();
            let confirming_post_delete = confirming_post_delete.clone();
            let navigator = navigator.clone();
            let error = error.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let post_id = post_id.clone();
                        async move { post_api::delete_post(&token, &post_id).await }
                    })
                    .await;
                match result {
                    Ok(()) => navigator.push(&Route::Profile),
                    Err(message) => error.set(Some(message.to_string())),
                }
                confirming_post_delete.set(false);
                deleting_post.set(false);
            });
        })
    };

    html! {
        <Wrapper classes="page post-detail">
            <Head classes="page__head post-detail__head" />
            <Body classes="page__body post-detail__body">
                if *loading {
                    <p class="post-detail__status blueprint-block" role="status">{"Loading story..."}</p>
                }
                if let Some(message) = &*error {
                    <p class="post-detail__error blueprint-block blueprint-block--error" role="alert">{message}</p>
                }
                if let Some(post) = &*post {
                    <div class="post-detail__content-wrapper">
                        if *editing_post {
                            <EditPostForm
                                post={post.clone()}
                                on_saved={on_post_saved.clone()}
                                on_cancel={on_cancel_post_edit.clone()}
                            />
                        } else {
                            <article class="post-detail__article blueprint-block">
                                <p class="post-detail__eyebrow">{"STORY"}</p>
                                if post.status == "draft" {
                                    <p class="post-detail__draft-label">{"DRAFT · ONLY YOU CAN SEE THIS STORY"}</p>
                                }
                                <div class="post-detail__actions">
                                    if current_user.as_ref().is_some_and(|user| user.id == post.author_id) {
                                        <button class="action--blue" type="button" onclick={on_edit_post}>
                                            {"Edit story"}
                                        </button>
                                        <button class="action--red" type="button" onclick={on_open_post_delete}>
                                            {"Delete story"}
                                        </button>
                                    }
                                </div>
                                <h1 class="post-detail__title">{post.title.clone()}</h1>
                                <p class="post-detail__meta">
                                    {format!("By @{} · {}", &post.author_id.to_string()[..8], post.created_at.format("%b %d, %Y"))}
                                </p>
                                <div class="post-detail__content">{content_text(post)}</div>
                                <div class="post-detail__tags">
                                    {for post.tags.iter().map(|tag| html! {
                                        <span class="post-card__tag" key={tag.id.to_string()}>{format!("#{}", tag.name)}</span>
                                    })}
                                </div>
                            </article>
                        }
                        if !*editing_post && post.status == "published" {
                            <PostRating
                                post_id={props.id.clone()}
                                rating_sum={post.rating_sum}
                                rating_count={post.rating_count}
                            />
                            <ReviewSection post_id={props.id.clone()} />
                        }
                    </div>
                }
            </Body>
            if *confirming_post_delete {
                <ConfirmDialog
                    title="Delete this story?"
                    message="This story will be deleted permanently from your profile."
                    confirm_label="Delete story"
                    confirm_classes={classes!("action--red")}
                    busy={*deleting_post}
                    on_confirm={on_confirm_post_delete.clone()}
                    on_cancel={on_cancel_post_delete.clone()}
                />
            }
        </Wrapper>
    }
}
