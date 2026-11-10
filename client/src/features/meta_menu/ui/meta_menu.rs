use yew::prelude::*;

use crate::{
    app::routes::Route,
    features::meta_menu::lib::{use_brand::use_brand, use_title::use_title},
    shared::ui::{menu::Menu, menu_link::MenuLink},
};

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
            <MenuLink classes="meta-menu__link" to={Route::Home}>
                { html!(brand) }
            </MenuLink>
        </Menu>
    }
}
