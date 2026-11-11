use yew::prelude::*;

use crate::{
    shared::ui::{block::Block, body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Wrapper classes="page not-found">
            <Head classes="page__head not-found__head" />
            <Body classes="page__body not-found__body">
                <Block classes="not-found__content">
                    {"Страница не найдена"}
                </Block>
            </Body>
        </Wrapper>
    }
}
