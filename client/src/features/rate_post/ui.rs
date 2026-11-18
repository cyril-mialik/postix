use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::post::PostRatingSummary;

use crate::{entities::post::post_api, features::auth::AuthContext};

#[derive(Properties, PartialEq)]
pub struct PostRatingProps {
    pub post_id: String,
    pub rating_sum: i32,
    pub rating_count: i32,
}

#[function_component(PostRating)]
pub fn post_rating(props: &PostRatingProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let summary = use_state(|| PostRatingSummary {
        rating_sum: props.rating_sum,
        rating_count: props.rating_count,
        my_rating: None,
    });
    let loading = use_state(|| true);
    let saving = use_state(|| false);
    let error = use_state(|| None::<String>);

    {
        let auth = auth.clone();
        let summary = summary.clone();
        let loading = loading.clone();
        let error = error.clone();
        let post_id = props.post_id.clone();
        let rating_sum = props.rating_sum;
        let rating_count = props.rating_count;
        use_effect_with(
            (auth.is_ready, post_id.clone(), rating_sum, rating_count),
            move |_| {
                summary.set(PostRatingSummary {
                    rating_sum,
                    rating_count,
                    my_rating: None,
                });
                error.set(None);
                loading.set(true);

                if auth.is_ready {
                    spawn_local(async move {
                        match auth
                            .with_refresh(|token| {
                                let post_id = post_id.clone();
                                async move { post_api::get_post_rating(&token, &post_id).await }
                            })
                            .await
                        {
                            Ok(result) => summary.set(result),
                            Err(message) => error.set(Some(message.to_string())),
                        }
                        loading.set(false);
                    });
                } else {
                    loading.set(false);
                }
                || ()
            },
        );
    }

    let on_rate = {
        let auth = auth.clone();
        let post_id = props.post_id.clone();
        let summary = summary.clone();
        let saving = saving.clone();
        let error = error.clone();
        Callback::from(move |rating: i32| {
            if *saving || !auth.is_authenticated() {
                return;
            }

            saving.set(true);
            error.set(None);
            let auth = auth.clone();
            let post_id = post_id.clone();
            let summary = summary.clone();
            let saving = saving.clone();
            let error = error.clone();
            spawn_local(async move {
                match auth
                    .with_refresh(|token| {
                        let post_id = post_id.clone();
                        async move { post_api::rate_post(&token, &post_id, rating).await }
                    })
                    .await
                {
                    Ok(result) => summary.set(result),
                    Err(message) => error.set(Some(message.to_string())),
                }
                saving.set(false);
            });
        })
    };

    let average = if summary.rating_count == 0 {
        "No ratings yet".to_owned()
    } else {
        format!(
            "{:.1} / 5 · {} ratings",
            summary.rating_sum as f64 / summary.rating_count as f64,
            summary.rating_count
        )
    };

    html! {
        <section class="post-rating blueprint-block" aria-labelledby="post-rating-heading">
            <div class="post-rating__summary">
                <h2 id="post-rating-heading">{"Story rating"}</h2>
                <p aria-live="polite">
                    {if *loading { "Loading rating..." } else { average.as_str() }}
                </p>
            </div>
            <div class="post-rating__options" aria-label="Rate this story from 1 to 5">
                {for (1..=5).map(|value| {
                    let on_rate = on_rate.clone();
                    let selected = summary.my_rating == Some(value);
                    html! {
                        <button
                            type="button"
                            aria-label={format!("Rate story {value} out of 5")}
                            aria-pressed={if selected { "true" } else { "false" }}
                            class={classes!(
                                "post-rating__button",
                                if selected { "action--active" } else { "" }
                            )}
                            disabled={*loading || *saving}
                            onclick={Callback::from(move |_| on_rate.emit(value))}
                        >
                            {value}
                        </button>
                    }
                })}
            </div>
            if let Some(message) = &*error {
                <p class="post-rating__error blueprint-block blueprint-block--error" role="alert">{message}</p>
            }
        </section>
    }
}
