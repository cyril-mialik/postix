use yew::prelude::*;
use yew_router::components::Link;

use crate::app::routes::Route;

#[derive(Properties, PartialEq)]
pub struct MenuLinkProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    pub to: Route,
}

#[function_component(MenuLink)]
pub fn menu_link(props: &MenuLinkProps) -> Html {
    html! {
        <Link<Route> to={props.to.clone()} classes={classes!("menu-link", props.classes.clone())}>
            { props.children.clone() }
        </Link<Route>>
    }
}
