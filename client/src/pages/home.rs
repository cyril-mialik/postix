use crate::components::{card::Card, layout::Layout};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Layout>
            <Card>
                <p>{"Estamos rodeados de pantallas, de nubes de datos y de presencias online, y la tecnología es un lenguaje que se entremezcla con el propio. Somos personas tanto físicas, como digitales. Aprendemos cada día a existir en ambos mundos. Lo digital es un espejo de lo que somos."}</p>
                <br />
                <p>{"El arte digital, también."}</p>
                <p>{"Es arte."}</p>
                <p>{"Es humano."}</p>
            </Card>

            <div class="grid-cards">
                <Card>
                    <p>{"Digital Impact es una exposición, pero, por encima de eso, es una experiencia; es una ciudad para explorar y un mapa donde poner nombre a aquellas personas que están creando el presente y el futuro del arte."}</p>
                </Card>
                <Card>
                    <p>{"Es una radiografía disruptiva del estado actual del arte y el diseño digital, una muestra de la fuerza creativa de una realidad llena de posibilidades."}</p>
                </Card>
            </div>
        </Layout>
    }
}
