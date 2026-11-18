use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::review::{ReviewReaction, ReviewResponse};

use crate::{entities::review::review_api, features::auth::AuthContext};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReviewSort {
    Newest,
    MostAgreed,
    MostDisagreed,
}

impl ReviewSort {
    fn from_value(value: &str) -> Self {
        match value {
            "most_agreed" => Self::MostAgreed,
            "most_disagreed" => Self::MostDisagreed,
            _ => Self::Newest,
        }
    }

    fn value(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::MostAgreed => "most_agreed",
            Self::MostDisagreed => "most_disagreed",
        }
    }
}

fn sort_reviews(reviews: &mut [ReviewResponse], sort: ReviewSort) {
    reviews.sort_by(|left, right| {
        right
            .is_mine
            .cmp(&left.is_mine)
            .then_with(|| match sort {
                ReviewSort::Newest => right.created_at.cmp(&left.created_at),
                ReviewSort::MostAgreed => right
                    .agree_count
                    .cmp(&left.agree_count)
                    .then_with(|| right.created_at.cmp(&left.created_at)),
                ReviewSort::MostDisagreed => right
                    .disagree_count
                    .cmp(&left.disagree_count)
                    .then_with(|| right.created_at.cmp(&left.created_at)),
            })
            .then_with(|| left.id.cmp(&right.id))
    });
}

#[derive(Properties, PartialEq)]
pub struct ReviewSectionProps {
    pub post_id: String,
}

#[function_component(ReviewSection)]
pub fn review_section(props: &ReviewSectionProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let reviews = use_state(Vec::<ReviewResponse>::new);
    let comment = use_state(String::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let reaction_review = use_state(|| None::<i32>);
    let sort = use_state(|| ReviewSort::Newest);

    {
        let reviews = reviews.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let post_id = props.post_id.clone();
        use_effect_with((auth.is_ready, post_id.clone()), move |_| {
            if auth.is_ready {
                spawn_local(async move {
                    let result = auth
                        .with_refresh(|token| {
                            let post_id = post_id.clone();
                            async move { review_api::get_reviews(&token, &post_id).await }
                        })
                        .await;
                    match result {
                        Ok(result) => reviews.set(result),
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

    let on_comment = {
        let comment = comment.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
            comment.set(input.value());
        })
    };

    let on_submit = {
        let auth = auth.clone();
        let post_id = props.post_id.clone();
        let comment = comment.clone();
        let reviews = reviews.clone();
        let error = error.clone();
        let saving = saving.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if *saving {
                return;
            }

            saving.set(true);
            error.set(None);
            let auth = auth.clone();
            let post_id = post_id.clone();
            let comment_value = (*comment).clone();
            let comment = comment.clone();
            let reviews = reviews.clone();
            let error = error.clone();
            let saving = saving.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let post_id = post_id.clone();
                        let comment = comment_value.clone();
                        async move { review_api::create_review(&token, &post_id, &comment).await }
                    })
                    .await;
                match result {
                    Ok(mut review) => {
                        review.is_mine = true;
                        let mut updated = (*reviews).clone();
                        updated.insert(0, review);
                        reviews.set(updated);
                        comment.set(String::new());
                    }
                    Err(message) => error.set(Some(message.to_string())),
                }
                saving.set(false);
            });
        })
    };

    let on_sort = {
        let sort = sort.clone();
        Callback::from(move |event: Event| {
            let select: web_sys::HtmlSelectElement = event.target_unchecked_into();
            sort.set(ReviewSort::from_value(&select.value()));
        })
    };

    let mut ordered_reviews = (*reviews).clone();
    sort_reviews(&mut ordered_reviews, *sort);

    let review_items = ordered_reviews.into_iter().map(|review| {
        let auth = auth.clone();
        let post_id = props.post_id.clone();
        let reviews = reviews.clone();
        let error = error.clone();
        let reaction_review = reaction_review.clone();
        let reaction_busy = reaction_review.clone();
        let review_id = review.id;
        let on_react = Callback::from(move |reaction: ReviewReaction| {
            if *reaction_review == Some(review_id) {
                return;
            }
            reaction_review.set(Some(review_id));
            error.set(None);
            let auth = auth.clone();
            let post_id = post_id.clone();
            let reviews = reviews.clone();
            let error = error.clone();
            let reaction_review = reaction_review.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let post_id = post_id.clone();
                        async move {
                            review_api::react_to_review(&token, &post_id, review_id, reaction)
                                .await
                        }
                    })
                    .await;
                match result {
                    Ok(summary) => {
                        let mut updated = (*reviews).clone();
                        if let Some(item) = updated.iter_mut().find(|item| item.id == review_id) {
                            item.agree_count = summary.agree_count;
                            item.disagree_count = summary.disagree_count;
                            item.my_reaction = summary.my_reaction;
                        }
                        reviews.set(updated);
                    }
                    Err(message) => error.set(Some(message.to_string())),
                }
                reaction_review.set(None);
            });
        });

        html! {
            <article
                id={format!("review-{}", review.id)}
                class="review-section__comment"
                key={review.id}
            >
                <div class="review-section__comment-meta">
                    <span>{format!("@{}", &review.author_id.to_string()[..8])}</span>
                    <time datetime={review.created_at.to_rfc3339()}>
                        {review.created_at.format("%b %d, %Y").to_string()}
                    </time>
                </div>
                <p class="review-section__comment-text">{review.review}</p>
                <div class="review-section__reactions">
                    <span>{format!("{} agree · {} disagree", review.agree_count, review.disagree_count)}</span>
                    <div
                        class="review-section__reaction-options"
                        role="group"
                        aria-label="React to this comment"
                    >
                        {for [ReviewReaction::Agree, ReviewReaction::Disagree].into_iter().map(|reaction| {
                            let on_react = on_react.clone();
                            let selected = review.my_reaction == Some(reaction);
                            let label = match reaction {
                                ReviewReaction::Agree => "Agree",
                                ReviewReaction::Disagree => "Disagree",
                            };
                            html! {
                                <button
                                    type="button"
                                    aria-label={format!("{label} with this comment")}
                                    aria-pressed={if selected { "true" } else { "false" }}
                                    class={classes!(
                                        "review-section__reaction-button",
                                        if selected { "action--active" } else { "" }
                                    )}
                                    disabled={*reaction_busy == Some(review_id)}
                                    onclick={Callback::from(move |_| on_react.emit(reaction))}
                                >
                                    {label}
                                </button>
                            }
                        })}
                    </div>
                </div>
            </article>
        }
    });

    let has_commented = reviews.iter().any(|review| review.is_mine);
    let own_review_id = reviews
        .iter()
        .find(|review| review.is_mine)
        .map(|review| review.id);

    html! {
        <section class="review-section blueprint-block" aria-labelledby="review-section-heading">
            <header class="review-section__header">
                <h2 id="review-section-heading">{"Comments"}</h2>
            </header>
            <div class="review-section__summary">
                <p class="review-section__description">{"Comments on this story"}</p>
                if has_commented {
                    <p class="review-section__notice">
                        {"You've already commented on this story. "}
                        if let Some(review_id) = own_review_id {
                            <a href={format!("#review-{review_id}")}>{"Jump to your comment"}</a>
                        }
                    </p>
                }
                if !reviews.is_empty() {
                    <div class="review-section__toolbar">
                        <label for="review-sort">{"Sort comments"}</label>
                        <select id="review-sort" value={sort.value()} onchange={on_sort}>
                            <option value="newest">{"Newest first"}</option>
                            <option value="most_agreed">{"Most agreed"}</option>
                            <option value="most_disagreed">{"Most disagreed"}</option>
                        </select>
                    </div>
                }
            </div>
            if *loading {
                <p class="review-section__status" role="status">{"Loading comments..."}</p>
            } else if reviews.is_empty() {
                <p class="review-section__empty">{"No comments yet."}</p>
            }
            if let Some(message) = &*error {
                <p class="review-section__error blueprint-block blueprint-block--error" role="alert">{message}</p>
            }
            <div class="review-section__comments">
                {for review_items}
            </div>
            if !has_commented {
                <form class="review-section__form" onsubmit={on_submit}>
                    <label for="post-comment">{"Add a comment"}</label>
                    <textarea
                        id="post-comment"
                        value={(*comment).clone()}
                        maxlength="5000"
                        rows="4"
                        required=true
                        disabled={*saving || *loading}
                        oninput={on_comment}
                    />
                    <button type="submit" disabled={*saving || *loading}>
                        {if *saving { "Adding comment..." } else { "Add comment" }}
                    </button>
                </form>
            }
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn review(
        id: i32,
        is_mine: bool,
        agree_count: i32,
        disagree_count: i32,
        day: u32,
    ) -> ReviewResponse {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "post_id": "00000000-0000-0000-0000-000000000000",
            "author_id": "00000000-0000-0000-0000-000000000000",
            "review": format!("Comment {id}"),
            "is_mine": is_mine,
            "agree_count": agree_count,
            "disagree_count": disagree_count,
            "my_reaction": null,
            "created_at": format!("2026-01-{day:02}T00:00:00Z"),
            "updated_at": format!("2026-01-{day:02}T00:00:00Z")
        }))
        .unwrap()
    }

    #[test]
    fn keeps_the_current_users_comment_first_and_sorts_the_rest_by_selection() {
        let mut reviews = vec![
            review(1, true, 0, 0, 1),
            review(2, false, 2, 5, 3),
            review(3, false, 4, 1, 2),
        ];

        sort_reviews(&mut reviews, ReviewSort::Newest);
        assert_eq!(
            reviews.iter().map(|item| item.id).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );

        sort_reviews(&mut reviews, ReviewSort::MostAgreed);
        assert_eq!(
            reviews.iter().map(|item| item.id).collect::<Vec<_>>(),
            vec![1, 3, 2]
        );

        sort_reviews(&mut reviews, ReviewSort::MostDisagreed);
        assert_eq!(
            reviews.iter().map(|item| item.id).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }
}
