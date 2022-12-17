// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::components::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

/// Defines the different endpoints which will be rendered by the main activity
/// given the path.
#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    /// The home URL, used to join chat rooms.
    #[at("/")]
    JoinRoom,
    /// The view where users will be able to discuss between each others.
    #[at("/r/:room")]
    Room { room: String },
    /// The place where a client can sign in to the application.
    #[at("/signin")]
    SignIn,
    /// The component on which the client can register himself.
    #[at("/signup")]
    SignUp,
    /// The user settings, on which he can modify his own profile.
    #[at("/settings")]
    Settings,
    /// Endpoint to log out the user.
    #[at("/logout")]
    LogOut,
    /// Any other route is redirected here.
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Function used to switch the main component's view.
pub fn switch(route: Route) -> Html {
    match route {
        Route::JoinRoom => html! { <JoinRoom /> },
        Route::Room { room } => html! { <Feed room={room} /> },
        Route::SignIn => html! { <SignIn /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Settings => html! { <Settings /> },
        Route::LogOut => html! { <LogOut /> },
        Route::NotFound => html! { <NotFound />},
    }
}

impl Route {
    /// Returns whether a route requires the user to be authenticated or not.
    pub fn requires_auth(&self) -> bool {
        matches!(self, Route::Room { room: _ } | Route::Settings)
    }
}
