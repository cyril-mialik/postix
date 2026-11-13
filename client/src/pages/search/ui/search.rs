use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <Wrapper classes="page search">
            <Head classes="page__head search__head" />
            <Body classes="page__body search__body">
                {"search"}
            </Body>
        </Wrapper>
    }
}
