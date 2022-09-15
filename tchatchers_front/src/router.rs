use crate::components::feed::Feed;
use crate::components::logout::LogOut;
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
    #[at("/logout")]
    LogOut,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Feed /> },
        Route::SignIn => html! { <SignIn /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Settings => html! { <Settings /> },
        Route::LogOut => html! { <LogOut /> },
    }
}

impl Route {
    pub fn requires_auth(&self) -> bool {
        match self {
            Route::Home | Route::Settings => true,
            _ => false,
        }
    }
}
