use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::layout::Layout;
use crate::app::routes::{Route, switch};
use crate::features::auth::lib::auth::AuthProvider;

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
