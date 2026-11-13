use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or(InputType::Text)]
    pub input_type: InputType,

    pub value: String,

    #[prop_or_default]
    pub on_change: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();

            on_change.emit(input.value());
        })
    };

    html! {
        <div class={classes!("input", props.classes.clone())}>
            <div class="input__container">
                if !props.label.is_empty() {
                    <label class="input__label">{ &props.label }</label>
                }

                <input
                    type={props.input_type.as_str()}
                    class="input__field"
                    value={props.value.clone()}
                    {oninput}
                />
            </div>
        </div>
    }
}
