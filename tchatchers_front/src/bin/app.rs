// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use tchatchers_core::app_context::AppContext;
use tchatchers_front::components::prelude::*;

use tchatchers_front::router::{switch, Route};
use tchatchers_front::utils::requester::Requester;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

#[function_component(ContextualApp)]
fn contextual_app() -> HtmlResult {
    let context = use_future(|| async {
        let req = Requester::<()>::get("/api/app_context");
        let resp = req.send().await;
        if resp.status().is_success() {
            let app_context: AppContext =
                serde_json::from_str(&resp.text().await.unwrap()).unwrap();
            Some(app_context)
        } else {
            None
        }
    })?;

    let context = use_state(|| context.clone());

    Ok(html! {
        <BrowserRouter>
            <ContextProvider<UseStateHandle<Option<AppContext>>> context={context}>
                <div class="h-screen grid grid-rows-12">
                    <NavbarHOC/>
                    <Toast />
                        <Modal />
                    <div class="row-span-11">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </ContextProvider<UseStateHandle<Option<AppContext>>>>
        </BrowserRouter>
    })
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Suspense fallback={html!{ <div>{"Loading ..."}</div>}}>
                <ContextualApp/>
            </Suspense>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
