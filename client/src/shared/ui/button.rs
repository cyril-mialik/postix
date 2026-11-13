use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
}

impl ButtonType {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={classes!("button", props.classes.clone())}
            type={props.button_type.as_str()}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            { &props.label }
        </button>
    }
}
