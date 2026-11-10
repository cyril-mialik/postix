use crate::{app::routes::Route, features::nav_menu::model::nav_menu_link::MenuLink};

pub const HOME: MenuLink = MenuLink {
    id: 1,
    name: "Home",
    to: Route::Home,
};

pub const PROFILE: MenuLink = MenuLink {
    id: 2,
    name: "Profile",
    to: Route::Profile,
};

pub const SEARCH: MenuLink = MenuLink {
    id: 3,
    name: "Search",
    to: Route::Search,
};
