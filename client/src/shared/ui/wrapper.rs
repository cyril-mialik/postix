use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WrapperProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Wrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {
    html! {
        <main class={classes!("wrapper", props.classes.clone())}>
            <div class="wrapper__container">
                { props.children.clone() }
            </div>
        </main>
    }
}
