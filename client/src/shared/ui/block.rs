use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Block)]
pub fn block(props: &BlockProps) -> Html {
    html! {
        <section class={classes!("block", props.classes.clone())}>
            { props.children.clone() }
        </section>
    }
}
