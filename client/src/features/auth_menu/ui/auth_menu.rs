use yew::prelude::*;

use crate::{
    app::AuthRoute,
    shared::{link::Link, menu::Menu},
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
                html! { <Link<AuthRoute> classes="auth-menu__link" key={item.id} to={item.to}>
                    {html! { item.name }}
                </Link<AuthRoute>> }
            }).collect::<Html>()}
        </Menu>
    }
}
