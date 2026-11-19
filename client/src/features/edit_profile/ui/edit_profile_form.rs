use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::user::UserResponse;

use crate::{entities::user::user_api, features::auth::AuthContext};

#[derive(Properties, PartialEq)]
pub struct EditProfileFormProps {
    pub user: UserResponse,
    pub on_saved: Callback<UserResponse>,
    pub on_cancel: Callback<()>,
}

#[function_component(EditProfileForm)]
pub fn edit_profile_form(props: &EditProfileFormProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let username = use_state(|| props.user.username.clone());
    let first_name = use_state(|| props.user.first_name.clone().unwrap_or_default());
    let last_name = use_state(|| props.user.last_name.clone().unwrap_or_default());
    let avatar = use_state(|| props.user.avatar.clone().unwrap_or_default());
    let error = use_state(|| None::<String>);
    let saving = use_state(|| false);

    let on_submit = {
        let username = username.clone();
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let avatar = avatar.clone();
        let error = error.clone();
        let saving = saving.clone();
        let auth = auth.clone();
        let on_saved = props.on_saved.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if *saving {
                return;
            }

            let request = match user_api::build_update_profile_request(
                &username,
                &first_name,
                &last_name,
                &avatar,
            ) {
                Ok(request) => request,
                Err(message) => {
                    error.set(Some(message));
                    return;
                }
            };

            error.set(None);
            saving.set(true);
            let auth = auth.clone();
            let error = error.clone();
            let saving = saving.clone();
            let on_saved = on_saved.clone();
            let request = request.clone();
            spawn_local(async move {
                let result = auth
                    .with_refresh(move |token| {
                        let request = request.clone();
                        async move { user_api::update_profile(&token, &request).await }
                    })
                    .await;
                match result {
                    Ok(user) => on_saved.emit(user),
                    Err(message) => error.set(Some(message.to_string())),
                }
                saving.set(false);
            });
        })
    };

    let on_username = {
        let username = username.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            username.set(input.value());
        })
    };
    let on_first_name = {
        let first_name = first_name.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            first_name.set(input.value());
        })
    };
    let on_last_name = {
        let last_name = last_name.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            last_name.set(input.value());
        })
    };
    let on_avatar = {
        let avatar = avatar.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            avatar.set(input.value());
        })
    };
    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <form class="profile-edit" onsubmit={on_submit}>
            <label for="profile-username">{"Username"}</label>
            <input
                id="profile-username"
                value={(*username).clone()}
                minlength="3"
                maxlength="128"
                required=true
                disabled={*saving}
                oninput={on_username}
            />
            <label for="profile-first-name">{"First name"}</label>
            <input
                id="profile-first-name"
                value={(*first_name).clone()}
                disabled={*saving}
                oninput={on_first_name}
            />
            <label for="profile-last-name">{"Last name"}</label>
            <input
                id="profile-last-name"
                value={(*last_name).clone()}
                disabled={*saving}
                oninput={on_last_name}
            />
            <label for="profile-avatar">{"Avatar URL"}</label>
            <input
                id="profile-avatar"
                type="url"
                value={(*avatar).clone()}
                disabled={*saving}
                oninput={on_avatar}
            />
            if let Some(message) = &*error {
                <p class="profile__error blueprint-block blueprint-block--error" role="alert">{message}</p>
            }
            <div class="profile-edit__actions">
                <button type="button" disabled={*saving} onclick={on_cancel}>{"Cancel"}</button>
                <button class="action--green" type="submit" disabled={*saving}>
                    {if *saving { "Saving..." } else { "Save profile" }}
                </button>
            </div>
        </form>
    }
}
