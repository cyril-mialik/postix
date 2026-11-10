use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BodyProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Body)]
pub fn body(props: &BodyProps) -> Html {
    html! {
        <section class={classes!("body", props.classes.clone())}>
            <div class="body__container">
                { props.children.clone() }
            </div>
        </section>
    }
}
