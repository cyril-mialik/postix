use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Wrapper classes="page home">
            <Head classes="page__head home__head" />
            <Body classes="page__body home__body">{"home"}</Body>
        </Wrapper>
    }
}
