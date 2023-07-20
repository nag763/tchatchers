use std::rc::Rc;

use yew::{function_component, html, use_context, BaseComponent, Html};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

use crate::{
    components::toast::Alert, router::Route, services::toast_bus::ToastBus,
    utils::client_context::ClientContext,
};

#[function_component(AuthGuard)]
pub fn auth_guard<T>(props: &<T as yew::BaseComponent>::Properties) -> Html
where
    T: BaseComponent,
    <T as yew::BaseComponent>::Properties: Clone,
{
    let client_context = use_context::<Rc<ClientContext>>().expect("No app context");
    let navigator = use_navigator().unwrap();



    if client_context.user.is_some() {
        html! { <T ..props.clone() /> }
    } else {
        navigator.replace(&Route::SignIn);
        ToastBus::dispatcher().send(Alert {
            is_success: false,
            content: client_context.translation.get_or_default("anon_guard", "Please authenticate prior accessing the app functionnalities"),
        });
        html! {<></>}
    }
}
