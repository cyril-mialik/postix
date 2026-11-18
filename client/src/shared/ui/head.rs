use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeadProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Head)]
pub fn head(props: &HeadProps) -> Html {
    html! {
        <header class={classes!("head", props.classes.clone())}>
            <div class="head__container">
                { props.children.clone() }
            </div>
        </header>
    }
}
