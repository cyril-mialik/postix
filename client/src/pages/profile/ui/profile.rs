use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <Wrapper classes="page profile">
            <Head classes="page__head profile__head" />
            <Body classes="page__body profile__body">
                {"profile"}
            </Body>
        </Wrapper>
    }
}
