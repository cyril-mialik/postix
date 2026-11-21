use postix_core::post::PostResponse;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::Route, entities::post::content_preview};

#[derive(Properties, PartialEq)]
pub struct PostCardProps {
    pub post: PostResponse,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let post = &props.post;
    let content = content_preview(post);
    let author = if post.author_username.is_empty() {
        post.author_id.to_string()[..8].to_owned()
    } else {
        post.author_username.clone()
    };
    let date = post.created_at.format("%b %d, %Y").to_string();

    html! {
        <Link<Route> to={Route::Post { id: post.id.to_string() }} classes="post-card__link">
            <article class="post-card blueprint-block">
                <div class="post-card__meta">
                    if let Some(avatar) = &post.author_avatar {
                        <img class="post-card__avatar" src={avatar.clone()} alt="" loading="lazy" />
                    }
                    <span class="post-card__author">{format!("@{author}")}</span>
                    <time datetime={post.created_at.to_rfc3339()}>{date}</time>
                    if post.status == "draft" {
                        <span class="post-card__status">{"DRAFT"}</span>
                    }
                </div>
                <h2 class="post-card__title">{post.title.clone()}</h2>
                <p class="post-card__excerpt">{content}</p>
                <div class="post-card__footer">
                    <div class="post-card__tags">
                        {for post.tags.iter().map(|tag| html! {
                            <span class="post-card__tag" key={tag.id}>{format!("#{}", tag.name)}</span>
                        })}
                    </div>
                    <span class="post-card__stats">
                        {format!("{} reads · {} reviews · ", post.views_count, post.reviews_count)}
                        {if post.rating_count == 0 {
                            "No ratings yet".to_owned()
                        } else {
                            format!(
                                "{:.1} / 5 · {} ratings",
                                post.rating_sum as f64 / post.rating_count as f64,
                                post.rating_count
                            )
                        }}
                    </span>
                </div>
            </article>
        </Link<Route>>
    }
}
