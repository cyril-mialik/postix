use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuLinkProps<R: Routable + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    pub to: R,
}

#[function_component(MenuLink)]
pub fn menu_link<R: Routable + PartialEq + 'static>(props: &MenuLinkProps<R>) -> Html {
    let current_route = use_route::<R>();
    let is_active = current_route.as_ref() == Some(&props.to);
    let mut link_classes = classes!("menu-link", props.classes.clone());

    if is_active {
        link_classes.push("menu-link--active");
    }

    html! {
        <Link<R> to={props.to.clone()} classes={classes!("menu-link", link_classes)}>
            { props.children.clone() }
        </Link<R>>
    }
}
