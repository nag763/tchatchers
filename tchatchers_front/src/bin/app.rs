// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use tchatchers_front::components::auth_checker::AuthChecker;
use tchatchers_front::components::modal::Modal;
use tchatchers_front::components::navbar::Navbar;
use tchatchers_front::components::toast::Toast;

use tchatchers_front::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>

            <BrowserRouter>
                <div class="h-screen grid grid-rows-12">
                    <Navbar/>
                    <Toast />
                        <Modal />
                    <div class="row-span-11">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            <AuthChecker />
            </BrowserRouter>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
