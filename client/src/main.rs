use crate::app::app::App;

mod app;
mod entities;
mod features;
mod pages;
mod shared;
mod widgets;

fn main() {
    yew::Renderer::<App>::new().render();
}
