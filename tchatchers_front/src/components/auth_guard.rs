use std::rc::Rc;

use yew::{function_component, html, use_context, BaseComponent, Html};
use yew_agent::worker::use_worker_subscription;
use yew_router::prelude::use_navigator;

use crate::{router::Route, utils::client_context::ClientContext};

use toast_service::{Alert, ToastBus};
#[function_component(AuthGuard)]
pub fn auth_guard<T>(props: &<T as yew::BaseComponent>::Properties) -> Html
where
    T: BaseComponent,
    <T as yew::BaseComponent>::Properties: Clone,
{
    let client_context = use_context::<Rc<ClientContext>>().expect("No app context");
    let navigator = use_navigator().unwrap();
    let toaster = use_worker_subscription::<ToastBus>();

    if client_context.user.is_some() {
        html! { <T ..props.clone() /> }
    } else {
        navigator.replace(&Route::SignIn);
        toaster.send(Alert {
            is_success: false,
            label: "anon_guard".into(),
            default: "Please authenticate prior accessing the app functionnalities".into(),
        });
        html! {<></>}
    }
}
