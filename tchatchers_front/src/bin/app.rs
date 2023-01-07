// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use tchatchers_core::user::PartialUser;
use tchatchers_front::components::prelude::*;

use tchatchers_front::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {

    let user = use_state(||
        tchatchers_front::utils::jwt::get_user().ok()
    ,
    );

    html! {
        <>
            <BrowserRouter>
                <ContextProvider<UseStateHandle<Option<PartialUser>>> context={user}>
                    <div class="h-screen grid grid-rows-12">
                        <NavbarHOC/>
                        <Toast />
                            <Modal />
                        <div class="row-span-11">
                            <Switch<Route> render={switch} />
                        </div>
                    </div>
                </ContextProvider<UseStateHandle<Option<PartialUser>>>>
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
