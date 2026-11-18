use yew::prelude::*;

use crate::{
    app::Route,
    shared::{menu::Menu, menu_link::MenuLink},
};

use crate::features::meta_menu::{use_brand, use_title};

#[derive(Properties, PartialEq)]
pub struct HeadMetaProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(MetaMenu)]
pub fn meta_menu(props: &HeadMetaProps) -> Html {
    let title = use_title();
    let brand = use_brand(title);

    html! {
        <Menu classes={classes!("meta-menu", props.classes.clone())}>
            <MenuLink<Route> classes="meta-menu__link" to={Route::About}>
                { html!(brand) }
            </MenuLink<Route>>
        </Menu>
    }
}
