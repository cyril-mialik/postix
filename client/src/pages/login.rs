use crate::components::{card::Card, layout::Layout};
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <Layout>
            <Card>
                <h2>{"Iniciar Sesión"}</h2>
                <br />
                <form style="display: flex; flex-direction: column; gap: 15px;">
                    <input type="email" placeholder="Email" style="padding: 10px; border: 1px solid blue; background: transparent;" />
                    <input type="password" placeholder="Password" style="padding: 10px; border: 1px solid blue; background: transparent;" />
                    <button type="button" style="padding: 10px; background: blue; color: white; border: none; cursor: pointer;">
                        {"ENTRAR"}
                    </button>
                </form>
            </Card>
        </Layout>
    }
}
