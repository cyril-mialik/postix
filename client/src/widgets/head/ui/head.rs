use yew::prelude::*;

use crate::{
    features::{
        auth::model::auth_context::AuthContext, auth_menu::ui::auth_menu::AuthMenu,
        meta_menu::ui::meta_menu::MetaMenu, nav_menu::ui::nav_menu::NavMenu,
    },
    shared::ui::head::Head as SharedHead,
};

#[derive(Properties, PartialEq)]
pub struct HeadProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Head)]
pub fn head(props: &HeadProps) -> Html {
    let auth = use_context::<AuthContext>().expect("AuthContext not found");
    let is_authenticated = auth.is_authenticated();

    html! {
        <SharedHead classes={classes!("head-widget", props.classes.clone())}>
            <MetaMenu classes="head-widget__meta" />

            if is_authenticated {
                <NavMenu classes="head-widget__nav" />
            } else {
                <AuthMenu classes="head-widget__auth" />
            }
        </SharedHead>
    }
}
