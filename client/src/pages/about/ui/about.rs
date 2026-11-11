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
                    {"
                        Это не было попыткой создать что-то коммерчески значимое или решить чью-то острую проблему.
                        Скорее, это был личный эксперимент - способ заглянуть за кулисы того,
                        что обычно скрыто от пользователя, и понять, из каких этапов складывается разработка собственного сайта.
                        Просто было интересно увидеть, как отдельные кусочки - вeрстка, логика, дизайн, деплой - складываются в единое целое,
                        и осознать, сколько на самом деле стоит за привычным \"сайт работает\".
                    "}
                </Block>
            </Body>
        </Wrapper>
    }
}
