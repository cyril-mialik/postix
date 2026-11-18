use yew::prelude::*;

use crate::{
    features::auth::SignUpForm,
    shared::{body::Body, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    html! {
        <Wrapper classes="page sign-up">
            <Head classes="page__head sign-up__head" />
            <Body classes="page__body sign-up__body">
                <SignUpForm />
            </Body>
        </Wrapper>
    }
}
