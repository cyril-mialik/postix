use yew::prelude::*;

use crate::{
    app::Route,
    shared::{link::Link, menu::Menu},
};

use super::super::config::nav_menu::NAV_MENU;

#[derive(Properties, PartialEq)]
pub struct NavMenuProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(NavMenu)]
pub fn nav_menu(props: &NavMenuProps) -> Html {
    html! {
        <Menu classes={classes!("nav-menu", props.classes.clone())}>
            {NAV_MENU.into_iter().map(|item| {
                let classes = classes!(
                    "nav-menu__link",
                    if item.is_primary { "nav-menu__link--primary action--green" } else { "" }
                );
                html! { <Link<Route> {classes} key={item.id} to={item.to}>{
                    html! { item.name }
                }</Link<Route>> }
            }).collect::<Html>()}
        </Menu>
    }
}
