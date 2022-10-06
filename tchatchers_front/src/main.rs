//! This part of the applicaiton is about the frontend
//!
//! The front end communicates through HTTP calls and WS with the frontend, and 
//! is built with WASM.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

/// The components of the application, used to render the views.
pub mod components;
/// The router, handles the switch between the views.
pub mod router;
/// The different services used to communicate with either the backend or the
/// components of the front end
pub mod services;
/// Functions used several times within the fronts.
pub mod utils;

use components::auth_checker::AuthChecker;
use components::navbar::Navbar;
use components::toast::Toast;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[macro_use]
extern crate derive_more;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <div class="h-screen grid grid-rows-12">
                    <Navbar/>
            <Toast />
                    <div class="row-span-11">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            <AuthChecker />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
