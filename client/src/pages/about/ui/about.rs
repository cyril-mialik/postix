use yew::prelude::*;

use crate::{
    shared::ui::{block::Block, body::Body, wrapper::Wrapper},
    widgets::head::ui::head::Head,
};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <Wrapper classes="page about">
            <Head classes="page__head about__head" />
            <Body classes="page__body about__body">
                <Block classes="about__content">
                    {"Данное приложение создавалось из интереса. Я просто хотел понять как выглядить полный цикл разработки своего сайта. Просто было интересно."}
                </Block>
            </Body>
        </Wrapper>
    }
}
