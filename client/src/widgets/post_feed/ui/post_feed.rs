use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    entities::post::{PostCard, matches_query, post_api},
    features::auth::AuthContext,
};

const PAGE_SIZE: usize = 20;

#[derive(Properties, PartialEq)]
pub struct PostFeedProps {
    #[prop_or_default]
    pub query: String,
    #[prop_or_default]
    pub refresh_key: u32,
}

#[function_component(PostFeed)]
pub fn post_feed(props: &PostFeedProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let posts = use_state(Vec::new);
    let offset = use_state(|| 0_usize);
    let loading = use_state(|| true);
    let loading_more = use_state(|| false);
    let has_more = use_state(|| false);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let offset = offset.clone();
        let loading = loading.clone();
        let has_more = has_more.clone();
        let error = error.clone();
        let auth = auth.clone();
        use_effect_with((auth.is_ready, props.refresh_key), move |_| {
            posts.set(Vec::new());
            offset.set(0);
            has_more.set(false);
            error.set(None);
            loading.set(true);

            if auth.is_ready {
                spawn_local(async move {
                    let result = auth
                        .with_refresh(|token| async move { post_api::get_feed(&token, 0).await })
                        .await;
                    match result {
                        Ok(result) => {
                            has_more.set(result.len() == PAGE_SIZE);
                            offset.set(result.len());
                            posts.set(result);
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

    let on_load_more = {
        let posts = posts.clone();
        let offset = offset.clone();
        let loading_more = loading_more.clone();
        let has_more = has_more.clone();
        let error = error.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            if !auth.is_authenticated() {
                return;
            }
            if *loading_more {
                return;
            }
            loading_more.set(true);
            let current_offset = *offset;
            let posts = posts.clone();
            let offset = offset.clone();
            let loading_more = loading_more.clone();
            let has_more = has_more.clone();
            let error = error.clone();
            let auth = auth.clone();

            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| async move {
                        post_api::get_feed(&token, current_offset).await
                    })
                    .await;
                match result {
                    Ok(result) => {
                        let count = result.len();
                        has_more.set(count == PAGE_SIZE);
                        let mut updated = (*posts).clone();
                        updated.extend(result);
                        posts.set(updated);
                        offset.set(current_offset + count);
                        error.set(None);
                    }
                    Err(message) => error.set(Some(message.to_string())),
                }
                loading_more.set(false);
            });
        })
    };

    let query = props.query.trim();
    let visible_posts = posts
        .iter()
        .filter(|post| matches_query(post, query))
        .cloned()
        .collect::<Vec<_>>();
    html! {
        <section class="post-feed" aria-label="Posts">
            if *loading {
                <p class="post-feed__status blueprint-block" role="status">{"Loading stories..."}</p>
            }
            if let Some(message) = &*error {
                <p class="post-feed__error blueprint-block blueprint-block--error" role="alert">{message}</p>
            }
            if !*loading && error.is_none() && visible_posts.is_empty() {
                <p class="post-feed__empty blueprint-block">
                    {if query.is_empty() { "No stories yet. Be the first to publish one." } else { "No stories match your search." }}
                </p>
            }
            {for visible_posts.into_iter().map(|post| html! {
                <PostCard post={post.clone()} key={post.id.to_string()} />
            })}
            if *has_more {
                <button
                    class="post-feed__more"
                    type="button"
                    disabled={*loading_more}
                    onclick={on_load_more}
                >
                    {if *loading_more { "Loading..." } else { "Load more stories" }}
                </button>
            }
        </section>
    }
}
