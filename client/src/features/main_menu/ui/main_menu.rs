use yew::prelude::*;

use crate::{
    app::Route,
    features::{auth::AuthContext, main_menu::model::menu_link::MenuLink},
    shared::{
        block::{Block, Tag},
        link::Link,
    },
};

use super::super::config::menu::{AUTH_MENU, UNAUTH_MENU};

#[derive(Properties, PartialEq)]
pub struct MainMenuProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(MainMenu)]
pub fn main_menu(props: &MainMenuProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let is_authenticated = auth.is_authenticated();

    let link = |item: MenuLink<Route>| {
        html! { <Link<Route> classes="main-menu__link" key={item.id} to={item.to}>
            {html! { item.name }}
        </Link<Route>> }
    };

    html! {
        <Block tag={Tag::Menu} classes={classes!("main-menu", props.classes.clone())}>
            if is_authenticated {
                {AUTH_MENU.into_iter().map(|item| { link(item) }).collect::<Html>()}
            } else {
                {UNAUTH_MENU.into_iter().map(|item| { link(item) }).collect::<Html>()}
            }
        </Block>
    }
}
