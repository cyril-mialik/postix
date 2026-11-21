use postix_core::report::ReportReason;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{entities::post::post_api, features::auth::AuthContext};

#[derive(Properties, PartialEq)]
pub struct PostActionsProps {
    pub post_id: String,
    #[prop_or_default]
    pub initially_bookmarked: bool,
}

fn reason_from_value(value: &str) -> ReportReason {
    match value {
        "abuse" => ReportReason::Abuse,
        "nsfw" => ReportReason::Nsfw,
        "violence" => ReportReason::Violence,
        "copyright" => ReportReason::Copyright,
        "other" => ReportReason::Other,
        _ => ReportReason::Spam,
    }
}

#[function_component(PostActions)]
pub fn post_actions(props: &PostActionsProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let bookmarked = use_state(|| props.initially_bookmarked);
    let report_open = use_state(|| false);
    let reason = use_state(|| "spam".to_owned());
    let message = use_state(String::new);
    let busy = use_state(|| false);
    let error = use_state(|| None::<String>);
    let notice = use_state(|| None::<String>);

    {
        let bookmarked = bookmarked.clone();
        use_effect_with(props.initially_bookmarked, move |initially_bookmarked| {
            bookmarked.set(*initially_bookmarked);
            || ()
        });
    }

    let on_bookmark = {
        let auth = auth.clone();
        let bookmarked = bookmarked.clone();
        let busy = busy.clone();
        let error = error.clone();
        let notice = notice.clone();
        let id = props.post_id.clone();
        Callback::from(move |_| {
            if *busy {
                return;
            }
            busy.set(true);
            error.set(None);
            let should_bookmark = !*bookmarked;
            let auth = auth.clone();
            let bookmarked = bookmarked.clone();
            let busy = busy.clone();
            let error = error.clone();
            let notice = notice.clone();
            let id = id.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let id = id.clone();
                        async move { post_api::set_bookmark(&token, &id, should_bookmark).await }
                    })
                    .await;
                match result {
                    Ok(()) => {
                        bookmarked.set(should_bookmark);
                        notice.set(Some(if should_bookmark {
                            "Saved to bookmarks.".to_owned()
                        } else {
                            "Removed from bookmarks.".to_owned()
                        }));
                    }
                    Err(message) => error.set(Some(message.to_string())),
                }
                busy.set(false);
            });
        })
    };

    let on_reason = {
        let reason = reason.clone();
        Callback::from(move |event: Event| {
            let select: web_sys::HtmlSelectElement = event.target_unchecked_into();
            reason.set(select.value());
        })
    };
    let on_message = {
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
            message.set(input.value());
        })
    };
    let on_report = {
        let auth = auth.clone();
        let reason = reason.clone();
        let message = message.clone();
        let busy = busy.clone();
        let error = error.clone();
        let notice = notice.clone();
        let report_open = report_open.clone();
        let id = props.post_id.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if *busy {
                return;
            }
            busy.set(true);
            error.set(None);
            let auth = auth.clone();
            let reason = reason_from_value(&reason);
            let message = (*message).clone();
            let busy = busy.clone();
            let error = error.clone();
            let notice = notice.clone();
            let report_open = report_open.clone();
            let id = id.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| {
                        let id = id.clone();
                        let message = message.clone();
                        async move { post_api::create_report(&token, &id, reason, &message).await }
                    })
                    .await;
                match result {
                    Ok(()) => {
                        report_open.set(false);
                        notice.set(Some("Report submitted.".to_owned()));
                    }
                    Err(message) => error.set(Some(message.to_string())),
                }
                busy.set(false);
            });
        })
    };

    html! {
        <div class="post-actions">
            <button type="button" class="post-actions__button" disabled={*busy} onclick={on_bookmark}>
                {if *bookmarked { "Remove bookmark" } else { "Bookmark" }}
            </button>
            <button
                type="button"
                class="post-actions__button post-actions__button--report"
                onclick={{
                    let report_open = report_open.clone();
                    Callback::from(move |_| report_open.set(true))
                }}
            >
                {"Report"}
            </button>
            if let Some(message) = &*notice {
                <span class="post-actions__notice" role="status">{message}</span>
            }
            if let Some(message) = &*error {
                <span class="post-actions__error" role="alert">{message}</span>
            }
            if *report_open {
                <form class="post-actions__report-form blueprint-block" onsubmit={on_report}>
                    <label for={format!("report-reason-{}", props.post_id)}>{"Reason"}</label>
                    <select
                        id={format!("report-reason-{}", props.post_id)}
                        value={(*reason).clone()}
                        onchange={on_reason}
                    >
                        <option value="spam">{"Spam"}</option>
                        <option value="abuse">{"Abuse"}</option>
                        <option value="nsfw">{"NSFW"}</option>
                        <option value="violence">{"Violence"}</option>
                        <option value="copyright">{"Copyright"}</option>
                        <option value="other">{"Other"}</option>
                    </select>
                    <label for={format!("report-message-{}", props.post_id)}>{"Details (optional)"}</label>
                    <textarea
                        id={format!("report-message-{}", props.post_id)}
                        maxlength="3000"
                        value={(*message).clone()}
                        oninput={on_message}
                    />
                    <button type="submit" disabled={*busy}>
                        {if *busy { "Submitting..." } else { "Submit report" }}
                    </button>
                    <button
                        type="button"
                        disabled={*busy}
                        onclick={{
                            let report_open = report_open.clone();
                            Callback::from(move |_| report_open.set(false))
                        }}
                    >
                        {"Cancel"}
                    </button>
                </form>
            }
        </div>
    }
}
