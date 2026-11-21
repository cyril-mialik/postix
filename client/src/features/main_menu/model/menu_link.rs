use yew_router::Routable;

pub struct MenuLink<T: Routable> {
    pub id: i32,
    pub name: &'static str,
    pub to: T,
}
