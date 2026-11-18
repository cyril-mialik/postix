use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    entities::post::{content_text, post_api},
    features::{auth::AuthContext, rate_post::PostRating},
    shared::{body::Body, wrapper::Wrapper},
    widgets::{head::Head, review_section::ReviewSection},
};

#[derive(Properties, PartialEq)]
pub struct PostDetailProps {
    pub id: String,
}

#[function_component(PostDetail)]
pub fn post_detail(props: &PostDetailProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let post = use_state(|| None);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let post = post.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let id = props.id.clone();
        use_effect_with((auth.is_ready, id.clone()), move |_| {
            post.set(None);
            error.set(None);
            loading.set(true);
            if auth.is_ready {
                spawn_local(async move {
                    let result = auth
                        .with_refresh(|token| {
                            let id = id.clone();
                            async move { post_api::get_post(&token, &id).await }
                        })
                        .await;
                    match result {
                        Ok(result) => post.set(Some(result)),
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
                        <article class="post-detail__article blueprint-block">
                            <p class="post-detail__eyebrow">{"STORY"}</p>
                            if post.status == "draft" {
                                <p class="post-detail__draft-label">{"DRAFT · ONLY YOU CAN SEE THIS STORY"}</p>
                            }
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
                        if post.status == "published" {
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
        </Wrapper>
    }
}
