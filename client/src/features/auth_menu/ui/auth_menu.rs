use yew::prelude::*;

use crate::{
    app::AuthRoute,
    shared::{menu::Menu, menu_link::MenuLink},
};

use super::super::config::auth_menu::AUTH_MENU;

#[derive(Properties, PartialEq)]
pub struct AuthMenuProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(AuthMenu)]
pub fn auth_menu(props: &AuthMenuProps) -> Html {
    html! {
        <Menu classes={classes!("auth-menu", props.classes.clone())}>
            {AUTH_MENU.into_iter().map(|item| {
                html! { <MenuLink<AuthRoute> classes="auth-menu__link" key={item.id} to={item.to}>{
                    html! { item.name }
                }</MenuLink<AuthRoute>> }
            }).collect::<Html>()}
        </Menu>
    }
}
