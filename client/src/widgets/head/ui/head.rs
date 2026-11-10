use yew::prelude::*;

use crate::{
    features::{meta_menu::ui::meta_menu::MetaMenu, nav_menu::ui::nav_menu::NavMenu},
    shared::ui::head::Head as SharedHead,
};

#[derive(Properties, PartialEq)]
pub struct HeadProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Head)]
pub fn head(props: &HeadProps) -> Html {
    html! {
        <SharedHead classes={classes!("head-widget", props.classes.clone())}>
            <MetaMenu classes="head-widget__meta" />
            <NavMenu classes="head-widget__nav" />
        </SharedHead>
    }
}
