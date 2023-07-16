// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::rc::Rc;

use tchatchers_core::user::PartialUser;
use tchatchers_front::components::prelude::*;

use tchatchers_front::router::{switch, Route};
use tchatchers_front::utils::client_context::ClientContext;
use tchatchers_front::utils::requester::Requester;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

#[function_component(ContextualApp)]
fn contextual_app() -> HtmlResult {
    let bearer: UseStateHandle<Option<String>> = use_state(|| None);

    let app_context = {
        let bearer_setter = bearer.setter();
        use_future(|| async {
            let mut req = Requester::get("/api/whoami");
            let resp = req.bearer_setter(bearer_setter).send().await;
            if resp.ok() {
                let user: PartialUser =
                    serde_json::from_str(&resp.text().await.unwrap()).unwrap();
                let user_context = user.try_into().unwrap();
                Some(user_context)
            } else {
                None
            }
        })?
    };

    let client_context = Rc::new(ClientContext {
        user_context: use_state(|| app_context.clone()),
        bearer,
    });

    let context = use_memo(|_| (*client_context).clone(), (*client_context).clone());

    Ok(html! {
        <BrowserRouter>
            <ContextProvider<Rc<ClientContext>> context={context}>
                <div class="h-screen grid grid-rows-12">
                    <NavbarHOC/>
                    <Toast />
                        <Modal />
                    <div class="row-span-11 overflow-y-auto">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
                <RightMenu />
            </ContextProvider<Rc<ClientContext>>>
        </BrowserRouter>
    })
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Suspense fallback={html!{ <div class="dark:text-white">{"Loading ..."}</div>}}>
                <ContextualApp/>
            </Suspense>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
