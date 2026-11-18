use yew::prelude::*;
use yew_router::hooks::use_route;

use crate::app::Route;

#[hook]
pub fn use_title() -> &str {
    let route = use_route::<Route>();

    match route {
        Some(Route::Home) => "home",
        Some(Route::Profile) => "profile",
        Some(Route::Search) => "search",
        Some(Route::CreatePost) => "write",
        Some(Route::Post { .. }) => "post",
        Some(Route::Auth) => "auth",
        Some(Route::NotFound) => "not-found",
        Some(Route::About) => "about",
        None => unreachable!(),
    }
}
