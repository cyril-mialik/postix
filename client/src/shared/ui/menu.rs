use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <menu class={classes!("menu", props.classes.clone())}>
            { props.children.clone() }
        </menu>
    }
}
