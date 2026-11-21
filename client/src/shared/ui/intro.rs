use yew::prelude::*;

use crate::shared::block::{Block, Tag};

#[derive(Properties, PartialEq)]
pub struct IntroProps {
    pub subtitle: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Intro)]
pub fn intro(props: &IntroProps) -> Html {
    html! {
        <Block tag={Tag::Header} classes={classes!("intro", props.classes.clone())}>
            <h1 class="intro__title">{props.title.clone()}</h1>
            <h2 class="intro__subtitle">{props.subtitle.clone()}</h2>
            <p class="intro__description">{props.description.clone()}</p>
        </Block>
    }
}
