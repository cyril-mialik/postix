use crate::app::routes::Route;

pub struct MenuLink {
    pub id: i32,
    pub name: &'static str,
    pub to: Route,
}
