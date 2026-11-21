use yew::prelude::*;

use crate::{
    features::{auth::AuthContext, auth_menu::AuthMenu, main_menu::MainMenu, meta_menu::MetaMenu},
    shared::head::Head as SharedHead,
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
                <MainMenu classes="head-widget__nav" />
            } else {
                <AuthMenu classes="head-widget__auth" />
            }
        </SharedHead>
    }
}
