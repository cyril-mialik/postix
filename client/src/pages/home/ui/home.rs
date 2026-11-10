use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, page::Page},
    widgets::head::ui::head::Head,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Page classes="app home">
            <Head classes="app__head home__head" />
            <Body classes="app__body home__body">{"home"}</Body>
        </Page>
    }
}
