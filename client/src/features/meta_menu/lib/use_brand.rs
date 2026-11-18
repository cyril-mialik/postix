use yew::prelude::*;

#[hook]
pub fn use_brand(name: &str) -> String {
    format!("POSTIX://{}", name)
}
