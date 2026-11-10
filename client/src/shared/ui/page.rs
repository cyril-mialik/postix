use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    html! {
        <main class={classes!("page", props.classes.clone())}>
            <div class="page__container">
                { props.children.clone() }
            </div>
        </main>
    }
}
