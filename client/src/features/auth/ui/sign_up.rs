use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::{AuthRoute, Route},
    features::auth::{AuthContext, auth_api, use_auth_form},
    shared::{
        block::Block,
        button::{Button, ButtonType},
        input::{Input, InputType},
    },
};

#[function_component(SignUpForm)]
pub fn sign_up_form() -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = use_navigator().unwrap();
    let (form_state, on_email, on_password, on_error, on_loading) = use_auth_form();

    let on_submit = {
        let auth = auth.clone();
        let form_state = form_state.clone();
        let on_error = on_error.clone();
        let on_loading = on_loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if form_state.loading {
                return;
            }

            let email = form_state.email.clone();
            let password = form_state.password.clone();

            on_loading.emit(true);
            on_error.emit(None);

            let auth = auth.clone();
            let on_error = on_error.clone();
            let on_loading = on_loading.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                match auth_api::sign_up(email, password).await {
                    Ok(token) => {
                        auth.login.emit(token);
                        navigator.push(&Route::Home);
                    }
                    Err(err) => {
                        on_error.emit(Some(err));
                    }
                }
                on_loading.emit(false);
            });
        })
    };

    html! {
        <Block classes="auth-form">
            <form class="sign-up-form" onsubmit={on_submit}>
                <header class="auth-form__header">
                    <p class="auth-form__eyebrow">{"Join the conversation"}</p>
                    <h1 class="auth-form__title">{"Create your account"}</h1>
                    <p class="auth-form__description">
                        {"Write, discover, and follow the ideas that matter to you."}
                    </p>
                </header>

                if let Some(err) = &form_state.error {
                    <p class="auth-form__error blueprint-block blueprint-block--error" role="alert" aria-live="polite">{ err }</p>
                }

                <div class="auth-form__fields">
                    <Input
                        id="sign-up-email"
                        name="email"
                        label="Email"
                        input_type={InputType::Email}
                        value={form_state.email.clone()}
                        placeholder="you@example.com"
                        autocomplete="email"
                        required={true}
                        disabled={form_state.loading}
                        on_change={on_email}
                    />

                    <Input
                        id="sign-up-password"
                        name="password"
                        label="Password"
                        input_type={InputType::Password}
                        value={form_state.password.clone()}
                        placeholder="At least 8 characters"
                        autocomplete="new-password"
                        required={true}
                        disabled={form_state.loading}
                        min_length={8}
                        max_length={128}
                        on_change={on_password}
                    />
                    <p class="auth-form__hint">{"Use 8–128 characters."}</p>
                </div>

                <Button
                    label={if form_state.loading { "Creating account..." } else { "Create account" }}
                    button_type={ButtonType::Submit}
                    disabled={form_state.loading}
                    classes="auth-form__submit"
                />

                <p class="auth-form__footer">
                    {"Already have an account? "}
                    <Link<AuthRoute> to={AuthRoute::SignIn} classes="auth-form__link">
                        {"Sign in"}
                    </Link<AuthRoute>>
                </p>
            </form>
        </Block>
    }
}
