use yew::prelude::*;
use yew_router::prelude::{Link as RouterLink, Routable, use_route};

#[derive(Properties, PartialEq)]
pub struct LinkProps<R: Routable + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    pub to: R,
}

#[function_component(Link)]
pub fn link<R: Routable + PartialEq + 'static>(props: &LinkProps<R>) -> Html {
    let current_route = use_route::<R>();
    let is_active = current_route.as_ref() == Some(&props.to);

    let mut link_classes = classes!("link", props.classes.clone());
    if is_active {
        link_classes.push("link--active");
    }

    html! {
        <RouterLink<R> to={props.to.clone()} classes={classes!("link", link_classes)}>
            { props.children.clone() }
        </RouterLink<R>>
    }
}
