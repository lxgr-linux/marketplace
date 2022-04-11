mod types;
mod components;
mod cardchain;

use yew::prelude::*;
use crate::components::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <body>
            { get_header() }
            <main role="main">
                <div class="album py-5 bg-light">
                    <div class="container">
                        <div class="row">
                            { get_offers() }
                        </div>
                    </div>
                </div>
            </main>
            { get_footer() }
        </body>
    }
}

fn main() {
    yew::start_app::<App>();
}
