use yew::prelude::*;

use crate::{
    shared::ui::{block::Block, body::Body, page::Page},
    widgets::head::ui::head::Head,
};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Page classes="app not-found">
            <Head classes="app__head not-found__head" />
            <Body classes="app__body not-found__body">
                <Block classes="not-found__content">
                    {"Страница не найдена"}
                </Block>
            </Body>
        </Page>
    }
}
