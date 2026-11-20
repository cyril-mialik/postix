use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IntroProps {
    pub subtitle: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Intro)]
pub fn intro(props: &IntroProps) -> Html {
    html! {
        <header class="intro blueprint-block">
            <p class="intro__subtitle">{props.subtitle.clone()}</p>
            <h1>{props.title.clone()}</h1>
            <p class="intro__description">{props.description.clone()}</p>

            if !props.children.is_empty() {
                <div class="intro__actions">{props.children.clone()}</div>
            }
        </header>
    }
}
