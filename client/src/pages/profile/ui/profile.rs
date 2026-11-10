use yew::prelude::*;

use crate::{
    shared::ui::{body::Body, page::Page},
    widgets::head::ui::head::Head,
};

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <Page classes="app profile">
            <Head classes="app__head profile__head" />
            <Body classes="app__body profile__body">
                {"profile"}
            </Body>
        </Page>
    }
}
