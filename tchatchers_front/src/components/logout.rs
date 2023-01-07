// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use gloo_net::http::Request;
use tchatchers_core::user::PartialUser;
use yew::{html, Html, function_component, use_context, UseStateHandle};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

#[function_component(LogOut)]
pub fn log_out_hoc() -> Html {
    
    let user = use_context::<UseStateHandle<Option<PartialUser>>>().expect("No user context");
    let navigator = use_navigator().unwrap();
    let req = Request::get("/api/logout").send();
    wasm_bindgen_futures::spawn_local(async move {
        req.await.unwrap();
        user.set(None);
        ToastBus::dispatcher().send(Alert {
            is_success: true,
            content: "You logged out with success".into(),
        });
        navigator.replace(&Route::SignIn);
    });
    html! {
        <div class="dark:bg-zinc-800">
        </div>
    }
}