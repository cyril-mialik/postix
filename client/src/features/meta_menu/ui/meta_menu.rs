use yew::prelude::*;

use crate::{
    app::Route,
    shared::{link::Link, menu::Menu},
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
            <Link<Route> classes="meta-menu__link" to={Route::About}>
                { html!(brand) }
            </Link<Route>>
        </Menu>
    }
}
