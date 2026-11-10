use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, page::Page},
    widgets::head::ui::head::Head,
};

#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <Page classes="app profile">
            <Head classes="app__head profile__head" />
            <Body classes="app__body profile__body">
                {"search"}
            </Body>
        </Page>
    }
}
