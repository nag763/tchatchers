use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use crate::utils::requester::Requester;
use crate::{components::toast::Alert, utils::client_context::ClientContext};
use yew::{function_component, html, use_context, Html};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

#[function_component(LogOut)]
pub fn log_out_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    let navigator = use_navigator().unwrap();
    let mut req = Requester::get("/api/logout");
    let translations = client_context.translation.clone();
    wasm_bindgen_futures::spawn_local(async move {
        req.send().await;
        ToastBus::dispatcher().send(Alert {
            is_success: true,
            content: translations
                .get_or_default("logged_out", "You logged out with success, see you !"),
        });
        client_context.user.set(None);
        client_context.bearer.set(None);
        navigator.replace(&Route::SignIn);
    });
    html! {
        <div class="dark:bg-zinc-800">
        </div>
    }
}
