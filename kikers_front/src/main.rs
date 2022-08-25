pub mod components;
pub mod router;
pub mod services;

use components::navbar::Navbar;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <div class="h-screen grid grid-rows-12">
                    <Navbar />
                    <div class="row-span-11">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
