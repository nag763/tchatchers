pub mod components;

use yew::prelude::*;
use components::{postbar::Postbar, navbar::Navbar, posts::Posts, footer::Footer};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="h-screen lg:h-auto grid lg:grid-none grid-rows-12 lg:grid-rows-none">
                <Navbar />
                <Postbar />
                <Posts />
                <Footer />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
