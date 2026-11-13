use yew::prelude::*;

use crate::{
    features::auth::ui::sign_up::SignUpForm, shared::ui::{body::Body, wrapper::Wrapper}, widgets::head::ui::head::Head,
};

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    html! {
        <Wrapper classes="page sign-un">
            <Head classes="page__head sign-un__head" />
            <Body classes="page__body sign-un__body">
                <SignUpForm />
            </Body>
        </Wrapper>
    }
}
