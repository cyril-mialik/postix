use crate::app::Route;

use super::super::model::nav_menu_link::MenuLink;

pub const HOME: MenuLink = MenuLink {
    id: 1,
    name: "Home",
    to: Route::Home,
    is_primary: false,
};

pub const PROFILE: MenuLink = MenuLink {
    id: 3,
    name: "Profile",
    to: Route::Profile,
    is_primary: false,
};

pub const SEARCH: MenuLink = MenuLink {
    id: 4,
    name: "Search",
    to: Route::Search,
    is_primary: false,
};

pub const WRITE: MenuLink = MenuLink {
    id: 2,
    name: "Create",
    to: Route::CreatePost,
    is_primary: true,
};
