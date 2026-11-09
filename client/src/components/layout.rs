use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="app-window">
            <div class="window-header">
                <div class="dots">
                    <div class="dot"></div>
                    <div class="dot"></div>
                    <div class="dot"></div>
                </div>
            </div>
            <div class="app-content">
                <nav class="nav-bar">
                    <div class="nav-links">
                        <Link<Route> to={Route::Home}>{"LA EXPOSICION"}</Link<Route>>
                        <a href="#">{"ARTISTAS"}</a>
                        <a href="#">{"INFO"}</a>
                        <a href="#">{"AGENDA"}</a>
                    </div>
                    <div class="logo">{"DIGITAL IMPACT"}</div>
                    <div class="nav-links">
                        <Link<Route> to={Route::Login}>{"LOGIN"}</Link<Route>>
                    </div>
                </nav>

                <div class="side-panel-left"></div>

                <main class="main-area">
                    { for props.children.iter() }
                </main>

                <div class="side-panel-right"></div>
            </div>
        </div>
    }
}
