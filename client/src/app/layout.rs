use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="layout layout--wrapper">
            <div class="layout__container">
                { props.children.clone() }
            </div>
        </div>
    }
}
