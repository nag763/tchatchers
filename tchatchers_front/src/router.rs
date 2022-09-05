use crate::components::feed::Feed;
use crate::components::settings::Settings;
use crate::components::signin::SignIn;
use crate::components::signup::SignUp;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signin")]
    SignIn,
    #[at("/signup")]
    SignUp,
    #[at("/settings")]
    Settings,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Feed /> },
        Route::SignIn => html! { <SignIn /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Settings => html! { <Settings /> },
    }
}
