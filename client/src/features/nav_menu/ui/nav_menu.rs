use yew::prelude::*;

use crate::{
    app::routes::Route,
    features::nav_menu::config::nav_menu::NAV_MENU,
    shared::ui::{menu::Menu, menu_link::MenuLink},
};

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
                html! { <MenuLink<Route> classes="nav-menu__link" key={item.id} to={item.to}>{
                    html! { item.name }
                }</MenuLink<Route>> }
            }).collect::<Html>()}
        </Menu>
    }
}
