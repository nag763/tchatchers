use std::rc::Rc;

use crate::components::common::Loading;
// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use crate::utils::requester::Requester;
use crate::{components::toast::Alert, utils::client_context::ClientContext};
use yew::suspense::use_future;
use yew::{function_component, html, use_context, Html, HtmlResult, Suspense};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

#[function_component(LogOut)]
pub fn log_out_hoc() -> Html {
    html! {
        <Suspense fallback={html!{<Loading/>}}>
            <LogOutFuture/>
        </Suspense>
    }
}

#[function_component(LogOutFuture)]
pub fn log_out_future() -> HtmlResult {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();
    let navigator = use_navigator().unwrap();
    let _ = use_future(|| async {
        let mut req = Requester::get("/api/logout");
        req.send().await;
        ToastBus::dispatcher().send(Alert {
            is_success: true,
            label: "logged_out".into(),
            default: "You logged out with success, see you !".into(),
        });
    });
    client_context.user.set(None);
    client_context.bearer.set(None);
    navigator.replace(&Route::SignIn);
    Ok(html! { <></> })
}
