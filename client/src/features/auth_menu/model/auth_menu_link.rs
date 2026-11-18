use crate::app::AuthRoute;

pub struct AuthMenuLink {
    pub id: i32,
    pub name: &'static str,
    pub to: AuthRoute,
}
