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

#[function_component(SignInForm)]
pub fn sign_in_form() -> Html {
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
                match auth_api::sign_in(email, password).await {
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
            <form class="sign-in-form" onsubmit={on_submit}>
                <header class="auth-form__header">
                    <p class="auth-form__eyebrow">{"Welcome back"}</p>
                    <h1 class="auth-form__title">{"Sign in to Postix"}</h1>
                    <p class="auth-form__description">{"Continue reading and sharing your stories."}</p>
                </header>

                if let Some(err) = &form_state.error {
                    <p class="auth-form__error blueprint-block blueprint-block--error" role="alert" aria-live="polite">{ err }</p>
                }

                <div class="auth-form__fields">
                    <Input
                        id="sign-in-email"
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
                        id="sign-in-password"
                        name="password"
                        label="Password"
                        input_type={InputType::Password}
                        value={form_state.password.clone()}
                        placeholder="Your password"
                        autocomplete="current-password"
                        required={true}
                        disabled={form_state.loading}
                        min_length={1}
                        on_change={on_password}
                    />
                </div>

                <Button
                    label={if form_state.loading { "Signing in..." } else { "Sign in" }}
                    button_type={ButtonType::Submit}
                    disabled={form_state.loading}
                    classes="auth-form__submit"
                />

                <p class="auth-form__footer">
                    {"New to Postix? "}
                    <Link<AuthRoute> to={AuthRoute::SignUp} classes="auth-form__link">
                        {"Create an account"}
                    </Link<AuthRoute>>
                </p>
            </form>
        </Block>
    }
}
