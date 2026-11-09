use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="card">
            { for props.children.iter() }
        </div>
    }
}
