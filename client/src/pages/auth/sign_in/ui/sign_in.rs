use yew::prelude::*;

use crate::{
    features::auth::ui::sign_in::SignInForm,
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    html! {
        <Wrapper classes="page sign-in">
            <Head classes="page__head sign-in__head" />
            <Body classes="page__body sign-in__body">
                <SignInForm />
            </Body>
        </Wrapper>
    }
}
