use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use postix_core::post::{PostPublicationStatus, PostResponse};
use postix_core::user::UserResponse;

use crate::{
    entities::post::{PostCard, post_api},
    entities::user::user_api,
    features::{
        auth::{AuthContext, auth_api},
        edit_profile::EditProfileForm,
    },
    shared::{body::Body, confirm_dialog::ConfirmDialog, intro::Intro, wrapper::Wrapper},
    widgets::head::Head,
};

const PAGE_SIZE: usize = 20;

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let user = use_state(|| None::<UserResponse>);
    let posts = use_state(Vec::<PostResponse>::new);
    let offset = use_state(|| 0_usize);
    let has_more = use_state(|| false);
    let loading_more = use_state(|| false);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);
    let active_tab = use_state(|| PostPublicationStatus::Published);
    let confirm_sign_out = use_state(|| false);
    let signing_out = use_state(|| false);
    let editing_profile = use_state(|| false);

    {
        let user = user.clone();
        let posts = posts.clone();
        let offset = offset.clone();
        let has_more = has_more.clone();
        let error = error.clone();
        let loading = loading.clone();
        let auth = auth.clone();
        let status = *active_tab;
        use_effect_with((auth.is_ready, status), move |_| {
            loading.set(true);
            posts.set(Vec::new());
            offset.set(0);
            has_more.set(false);
            error.set(None);
            if auth.is_ready {
                spawn_local(async move {
                    let profile_result = auth
                        .with_refresh(|token| async move { user_api::get_me(&token).await })
                        .await;
                    match profile_result {
                        Ok(profile) => {
                            user.set(Some(profile));
                            let result = auth
                                .with_refresh(|token| async move {
                                    post_api::get_my_posts(&token, 0, status).await
                                })
                                .await;
                            match result {
                                Ok(result) => {
                                    has_more.set(result.len() == PAGE_SIZE);
                                    offset.set(result.len());
                                    posts.set(result);
                                }
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

    let on_load_more = {
        let posts = posts.clone();
        let offset = offset.clone();
        let has_more = has_more.clone();
        let loading_more = loading_more.clone();
        let error = error.clone();
        let auth = auth.clone();
        let status = *active_tab;
        Callback::from(move |_| {
            if !*has_more || *loading_more || !auth.is_authenticated() {
                return;
            }

            loading_more.set(true);
            let current_offset = *offset;
            let posts = posts.clone();
            let offset = offset.clone();
            let has_more = has_more.clone();
            let loading_more = loading_more.clone();
            let error = error.clone();
            let auth = auth.clone();

            spawn_local(async move {
                let result = auth
                    .with_refresh(|token| async move {
                        post_api::get_my_posts(&token, current_offset, status).await
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

    let on_select_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |status: PostPublicationStatus| active_tab.set(status))
    };

    let on_open_sign_out = {
        let confirm_sign_out = confirm_sign_out.clone();
        Callback::from(move |_| confirm_sign_out.set(true))
    };

    let on_cancel_sign_out = {
        let confirm_sign_out = confirm_sign_out.clone();
        Callback::from(move |_| confirm_sign_out.set(false))
    };

    let on_confirm_sign_out = {
        let auth = auth.clone();
        let confirm_sign_out = confirm_sign_out.clone();
        let signing_out = signing_out.clone();
        let error = error.clone();
        Callback::from(move |_| {
            if *signing_out {
                return;
            }

            signing_out.set(true);
            error.set(None);
            let auth = auth.clone();
            let confirm_sign_out = confirm_sign_out.clone();
            let signing_out = signing_out.clone();
            let error = error.clone();
            spawn_local(async move {
                match auth_api::sign_out().await {
                    Ok(()) => auth.logout.emit(()),
                    Err(message) => {
                        error.set(Some(message));
                        confirm_sign_out.set(false);
                    }
                }
                signing_out.set(false);
            });
        })
    };

    let on_published_tab = {
        let on_select_tab = on_select_tab.clone();
        Callback::from(move |_| on_select_tab.emit(PostPublicationStatus::Published))
    };
    let on_draft_tab =
        { Callback::from(move |_| on_select_tab.emit(PostPublicationStatus::Draft)) };
    let on_edit_profile = {
        let editing_profile = editing_profile.clone();
        Callback::from(move |_| editing_profile.set(true))
    };
    let on_cancel_profile_edit = {
        let editing_profile = editing_profile.clone();
        Callback::from(move |_| editing_profile.set(false))
    };
    let on_profile_saved = {
        let user = user.clone();
        let editing_profile = editing_profile.clone();
        Callback::from(move |updated: UserResponse| {
            user.set(Some(updated));
            editing_profile.set(false);
        })
    };

    html! {
        <Wrapper classes="page profile">
            <Head classes="page__head profile__head" />
            <Body classes="page__body profile__body">
                <div class="profile__content">
                    <Intro
                        subtitle="YOUR ACCOUNT"
                        title="Profile"
                        description="Manage your account and keep track of your published stories and drafts."
                    />
                    if *loading {
                        <p role="status">{"Loading profile..."}</p>
                    }
                    if let Some(message) = &*error {
                        <p class="profile__error blueprint-block blueprint-block--error" role="alert">{message}</p>
                    }
                    if let Some(user) = &*user {
                        <section class="profile__card blueprint-block" aria-label="Your profile details">
                            <div class="profile__actions">
                                if !*editing_profile {
                                    <button
                                        type="button"
                                        class="action--green"
                                        onclick={on_edit_profile}
                                    >
                                        {"Edit profile"}
                                    </button>
                                }
                                <button
                                    type="button"
                                    class="profile__sign-out action--red"
                                    disabled={*signing_out}
                                    onclick={on_open_sign_out}
                                >
                                    {"Sign out"}
                                </button>
                            </div>
                            if *editing_profile {
                                <EditProfileForm
                                    user={(*user).clone()}
                                    on_saved={on_profile_saved.clone()}
                                    on_cancel={on_cancel_profile_edit.clone()}
                                />
                            } else {
                                if let Some(avatar) = &user.avatar {
                                    <img class="profile__avatar" src={avatar.clone()} alt="" />
                                } else {
                                    <div class="profile__avatar" aria-hidden="true">
                                        {user.username.chars().next().unwrap_or('P').to_uppercase().to_string()}
                                    </div>
                                }
                                <div class="profile__identity">
                                    <h2>{format!("@{}", user.username)}</h2>
                                    <p>{user.email.clone()}</p>
                                </div>
                            }
                            <dl class="profile__stats">
                                <div><dt>{"Stories"}</dt><dd>{user.posts_count}</dd></div>
                                <div><dt>{"Followers"}</dt><dd>{user.followers_count}</dd></div>
                                <div><dt>{"Following"}</dt><dd>{user.following_count}</dd></div>
                            </dl>
                        </section>
                        <section class="profile__stories" aria-labelledby="profile-stories-heading">
                            <div class="profile__stories-header blueprint-block">
                                <h2 id="profile-stories-heading">{"Your stories"}</h2>
                                <div class="profile__tabs" role="tablist" aria-label="Your stories">
                                <button
                                    type="button"
                                    role="tab"
                                    aria-selected={if *active_tab == PostPublicationStatus::Published { "true" } else { "false" }}
                                    class={classes!(
                                        "profile__tab",
                                        if *active_tab == PostPublicationStatus::Published { "action--active" } else { "" }
                                    )}
                                    onclick={on_published_tab}
                                >
                                    {"Published"}
                                </button>
                                <button
                                    type="button"
                                    role="tab"
                                    aria-selected={if *active_tab == PostPublicationStatus::Draft { "true" } else { "false" }}
                                    class={classes!(
                                        "profile__tab",
                                        if *active_tab == PostPublicationStatus::Draft { "action--active" } else { "" }
                                    )}
                                    onclick={on_draft_tab}
                                >
                                    {"Draft"}
                                </button>
                                </div>
                            </div>
                            if !*loading && posts.is_empty() {
                                <p class="profile__stories-empty blueprint-block">
                                    {if *active_tab == PostPublicationStatus::Published {
                                        "You haven't published any stories yet."
                                    } else {
                                        "You don't have any drafts yet."
                                    }}
                                </p>
                            } else {
                                {for posts.iter().cloned().map(|post| html! {
                                    <PostCard post={post.clone()} key={post.id.to_string()} />
                                })}
                            }
                            if *has_more {
                                <button
                                    type="button"
                                    disabled={*loading_more}
                                    onclick={on_load_more}
                                >
                                    {if *loading_more { "Loading..." } else { "Load more stories" }}
                                </button>
                            }
                        </section>
                        if *confirm_sign_out {
                            <ConfirmDialog
                                title="Sign out?"
                                message="You will need to sign in again to access your profile."
                                confirm_label="Sign out"
                                confirm_classes="action--red"
                                busy={*signing_out}
                                on_confirm={on_confirm_sign_out}
                                on_cancel={on_cancel_sign_out}
                            />
                        }
                    }
                </div>
            </Body>
        </Wrapper>
    }
}
