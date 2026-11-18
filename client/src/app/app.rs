use yew::prelude::*;
use yew_router::prelude::*;

use super::{
    Layout,
    routes::{Route, switch},
};
use crate::features::auth::AuthProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Layout>
            <AuthProvider>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </AuthProvider>
        </Layout>
    }
}
