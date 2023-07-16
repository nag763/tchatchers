
use tchatchers_core::app_context::AppContext;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::components::prelude::*;

use crate::router::{switch, Route};
use crate::utils::requester::Requester;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

#[function_component(ContextualApp)]
fn contextual_app() -> HtmlResult {
    let context = use_future(|| async {
        let req = Requester::<()>::get("/api/whoami");
        let resp = req.send().await;
        if resp.status().is_success() {
            let user: PartialUser =
                serde_json::from_str(&resp.text().await.unwrap()).unwrap();
                let app_context = user.try_into().unwrap();
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
                    <RMenu/>
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

#[wasm_bindgen]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}