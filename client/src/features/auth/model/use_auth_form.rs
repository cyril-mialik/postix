use yew::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct AuthFormState {
    pub email: String,
    pub password: String,
    pub error: Option<String>,
    pub loading: bool,
}

#[hook]
pub fn use_auth_form() -> (
    UseStateHandle<AuthFormState>,
    Callback<String>,
    Callback<String>,
    Callback<Option<String>>,
    Callback<bool>,
) {
    let state = use_state(AuthFormState::default);

    let on_email = {
        let state = state.clone();
        Callback::from(move |email: String| {
            let mut new_state = (*state).clone();
            new_state.email = email;
            state.set(new_state);
        })
    };

    let on_password = {
        let state = state.clone();
        Callback::from(move |password: String| {
            let mut new_state = (*state).clone();
            new_state.password = password;
            state.set(new_state);
        })
    };

    let on_error = {
        let state = state.clone();
        Callback::from(move |err: Option<String>| {
            let mut new_state = (*state).clone();
            new_state.error = err;
            state.set(new_state);
        })
    };

    let on_loading = {
        let state = state.clone();
        Callback::from(move |loading: bool| {
            let mut new_state = (*state).clone();
            new_state.loading = loading;
            state.set(new_state);
        })
    };

    (state, on_email, on_password, on_error, on_loading)
}
