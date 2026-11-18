use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    features::auth::{GuestOnly, RequireAuth},
    pages::{
        about::About,
        auth::{SignIn, SignUp},
        create_post::CreatePostPage,
        home::Home,
        not_found::NotFound,
        post::PostDetail,
        profile::Profile,
        search::Search,
    },
};

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute {
    #[at("/auth/sign-in")]
    SignIn,

    #[at("/auth/sign-up")]
    SignUp,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    About,

    #[at("/home")]
    Home,

    #[at("/search")]
    Search,

    #[at("/profile")]
    Profile,

    #[at("/posts/new")]
    CreatePost,

    #[at("/posts/:id")]
    Post { id: String },

    #[at("/auth/*")]
    Auth,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_auth(route: AuthRoute) -> Html {
    match route {
        AuthRoute::SignIn => html! {
            <GuestOnly>
                <SignIn />
            </GuestOnly>
        },
        AuthRoute::SignUp => html! {
            <GuestOnly>
                <SignUp />
            </GuestOnly>
        },
        AuthRoute::NotFound => html! {
            <RequireAuth>
                <NotFound />
            </RequireAuth>
        },
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <RequireAuth>
                <Home />
            </RequireAuth>
        },
        Route::Search => html! {
            <RequireAuth>
                <Search />
            </RequireAuth>
        },
        Route::Profile => html! {
            <RequireAuth>
                <Profile />
            </RequireAuth>
        },
        Route::CreatePost => html! {
            <RequireAuth>
                <CreatePostPage />
            </RequireAuth>
        },
        Route::Post { id } => html! {
            <RequireAuth>
                <PostDetail {id} />
            </RequireAuth>
        },
        Route::About => html! { <About /> },
        Route::Auth => html! { <Switch<AuthRoute> render={switch_auth} /> },
        Route::NotFound => html! {
            <RequireAuth>
                <NotFound />
            </RequireAuth>
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_post_route_precedes_the_dynamic_post_route() {
        assert!(matches!(
            Route::recognize("/posts/new"),
            Some(Route::CreatePost)
        ));
        assert!(matches!(
            Route::recognize("/posts/abc"),
            Some(Route::Post { id }) if id == "abc"
        ));
    }
}
