use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::Route;
use crate::features::auth::{
    api::auth_api,
    model::{auth_context::AuthContext, use_auth_form::use_auth_form},
};
use crate::shared::ui::{
    button::Button,
    input::{Input, InputType},
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

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
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
        <div class="sign-up-form">
            <div class="sign-up-form__container">
                <legend class="sign-up-form__legend">{"Sign Up"}</legend>

                if let Some(err) = &form_state.error {
                    <div class="sign-up-form-error">{ err }</div>
                }

                <Input
                    label="Email"
                    input_type={InputType::Email}
                    value={form_state.email.clone()}
                    on_change={on_email}
                />

                <Input
                    label="Password"
                    input_type={InputType::Password}
                    value={form_state.password.clone()}
                    on_change={on_password}
                />

                <Button
                    label="Submit"
                    disabled={form_state.loading}
                    onclick={on_submit}
                />
            </div>
        </div>
    }
}
