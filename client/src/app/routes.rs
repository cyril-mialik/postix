use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    about::ui::about::About, auth::{sign_in::ui::sign_in::SignIn, sign_up::ui::sign_up::SignUp}, home::ui::home::Home, not_found::ui::not_found::NotFound, profile::ui::profile::Profile, search::ui::search::Search,
};

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute {
    #[at("/sign-in")]
    SignIn,

    #[at("/sign-up")]
    SignUp,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    About,

    #[at("/home")]
    Home,

    #[at("/search")]
    Search,

    #[at("/profile")]
    Profile,

    #[at("/auth")]
    Auth,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_auth(route: AuthRoute) -> Html {
    match route {
        AuthRoute::SignIn => html! { <SignIn /> },
        AuthRoute::SignUp => html! { <SignUp /> },
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Search => html! { <Search /> },
        Route::Profile => html! { <Profile /> },
        Route::About => html! { <About /> },
        Route::Auth => html! { <Switch<AuthRoute> render={switch_auth} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
