use crate::app::{Route};

use super::super::model::menu_link::MenuLink;

pub const HOME: MenuLink<Route> = MenuLink {
    id: 1,
    name: "Home",
    to: Route::Home,
};

pub const PROFILE: MenuLink<Route> = MenuLink {
    id: 2,
    name: "Profile",
    to: Route::Profile,
};

pub const SEARCH: MenuLink<Route> = MenuLink {
    id: 3,
    name: "Search",
    to: Route::Search,
};

pub const CREATE: MenuLink<Route> = MenuLink {
    id: 4,
    name: "Create",
    to: Route::CreatePost,
};

pub const ABOUT: MenuLink<Route> = MenuLink {
    id: 5,
    name: "About",
    to: Route::About,
};
