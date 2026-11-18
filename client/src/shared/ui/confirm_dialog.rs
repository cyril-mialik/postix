use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    #[prop_or("Cancel".to_owned())]
    pub cancel_label: String,
    #[prop_or_default]
    pub confirm_classes: Classes,
    #[prop_or_default]
    pub busy: bool,
    #[prop_or_default]
    pub on_confirm: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let on_confirm = {
        let callback = props.on_confirm.clone();
        Callback::from(move |_| callback.emit(()))
    };
    let on_cancel = {
        let callback = props.on_cancel.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <div class="confirm-dialog__backdrop">
            <section
                class="confirm-dialog blueprint-block"
                role="dialog"
                aria-modal="true"
                aria-labelledby="confirm-dialog-title"
            >
                <h2 id="confirm-dialog-title">{props.title.clone()}</h2>
                <p>{props.message.clone()}</p>
                <div class="confirm-dialog__actions">
                    <button type="button" class="confirm-dialog__cancel" disabled={props.busy} onclick={on_cancel}>
                        {props.cancel_label.clone()}
                    </button>
                    <button
                        type="button"
                        class={classes!("confirm-dialog__confirm", props.confirm_classes.clone())}
                        disabled={props.busy}
                        onclick={on_confirm}
                    >
                        {if props.busy { "Please wait..." } else { props.confirm_label.as_str() }}
                    </button>
                </div>
            </section>
        </div>
    }
}
