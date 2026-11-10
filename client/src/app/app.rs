use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::layout::Layout;
use crate::app::routes::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Layout>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </Layout>
    }
}
