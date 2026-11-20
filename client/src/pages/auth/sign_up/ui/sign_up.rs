use yew::prelude::*;

use crate::{
    features::auth::SignUpForm,
    shared::{body::Body, wrapper::Wrapper},
    widgets::head::Head,
};

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    html! {
        <Wrapper classes="page">
            <Head classes="page__head" />
            <Body classes="page__body">
                <SignUpForm />
            </Body>
        </Wrapper>
    }
}
