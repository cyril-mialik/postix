use crate::features::nav_menu::{
    config::nav_menu_item::{HOME, PROFILE, SEARCH},
    model::nav_menu_link::MenuLink,
};

pub const NAV_MENU: [MenuLink; 3] = [HOME, SEARCH, PROFILE];
