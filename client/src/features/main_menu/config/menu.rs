use crate::{
    app::Route,
    features::main_menu::{
        config::link::{ABOUT, CREATE, HOME, PROFILE, SEARCH},
        model::menu_link::MenuLink,
    },
};

pub const AUTH_MENU: [MenuLink<Route>; 5] = [HOME, SEARCH, PROFILE, CREATE, ABOUT];
pub const UNAUTH_MENU: [MenuLink<Route>; 3] = [HOME, SEARCH, ABOUT];
