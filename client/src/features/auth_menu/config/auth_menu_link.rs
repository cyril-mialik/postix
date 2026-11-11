use crate::{app::routes::AuthRoute, features::auth_menu::model::auth_menu_link::AuthMenuLink};

pub const SIGN_IN: AuthMenuLink = AuthMenuLink {
    id: 1,
    name: "Sign In",
    to: AuthRoute::SignIn,
};

pub const SIGN_UP: AuthMenuLink = AuthMenuLink {
    id: 2,
    name: "Sign Up",
    to: AuthRoute::SignUp,
};
