use yew::prelude::*;

#[derive(PartialEq)]
pub enum Tag {
    Div,
    Header,
    Footer,
    Section,
    Menu,
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(Tag::Div)]
    pub tag: Tag,
}

#[function_component(Block)]
pub fn block(props: &BlockProps) -> Html {
    match props.tag {
        Tag::Section => html! {
            <section class={classes!("block", props.classes.clone())}>
                { for props.children.iter() }
            </section>
        },
        Tag::Header => html! {
            <header class={classes!("block", props.classes.clone())}>
                { for props.children.iter() }
            </header>
        },
        Tag::Footer => html! {
            <footer class={classes!("block", props.classes.clone())}>
                { for props.children.iter() }
            </footer>
        },
        Tag::Div => html! {
            <div class={classes!("block", props.classes.clone())}>
                { for props.children.iter() }
            </div>
        },
        Tag::Menu => html! {
            <menu class={classes!("block", props.classes.clone())}>
                { for props.children.iter() }
            </menu>
        },
    }
}
