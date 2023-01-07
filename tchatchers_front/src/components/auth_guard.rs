use tchatchers_core::app_context::AppContext;
use yew::{function_component, html, use_context, BaseComponent, Html, UseStateHandle};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

use crate::{components::toast::Alert, router::Route, services::toast_bus::ToastBus};

#[function_component(AuthGuard)]
pub fn auth_guard<T>(props: &<T as yew::BaseComponent>::Properties) -> Html
where
    T: BaseComponent,
    <T as yew::BaseComponent>::Properties: Clone,
{
    let app_context = use_context::<UseStateHandle<Option<AppContext>>>().expect("No app context");
    let navigator = use_navigator().unwrap();

    match &*app_context {
        Some(_app_context) => html! { <T ..props.clone() /> },
        None => {
            navigator.replace(&Route::SignIn);
            ToastBus::dispatcher().send(Alert {
                is_success: false,
                content: "Please authenticate prior accessing the app functionnalities.".into(),
            });
            html! {<></>}
        }
    }
}
