
use tchatchers_core::user::PartialUser;
use yew::{function_component, Html, html, use_context, UseStateHandle, BaseComponent};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;

use crate::{services::toast_bus::ToastBus, router::Route, components::toast::Alert};


#[function_component(AuthGuard)]
pub fn auth_guard<T>(props: &<T as yew::BaseComponent>::Properties) -> Html where T: BaseComponent, <T as yew::BaseComponent>::Properties : Clone {
    let user = use_context::<UseStateHandle<Option<PartialUser>>>().expect("No user context");
    let navigator = use_navigator().unwrap();

    match &*user {
        Some(_user) => html! { <T ..props.clone() /> },
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