// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::components::{
    common::VerificationSucceeded, join_room::JoinRoomHOC, prelude::*, signup::SignUpHOC,
};
use yew::{html, Html, Suspense};
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
    #[at("/verify/:token")]
    Verify { token: String },
    #[at("/verification_failed")]
    VerificationFailed,
    #[at("/verification_succeeded")]
    VerificationSucceeded,
    #[at("/contact")]
    Contact,
    #[at("/gdpr")]
    GDPR,
}

/// Function used to switch the main component's view.
pub fn switch(route: Route) -> Html {
    match route {
        Route::JoinRoom => html! { <AuthGuard<JoinRoomHOC> /> },
        Route::Room { room } => html! { <AuthGuard<FeedHOC> {room} /> },
        Route::SignIn => html! { <SignInHOC /> },
        Route::SignUp => html! { <SignUpHOC /> },
        Route::Settings => html! { <AuthGuard<SettingsHOC> /> },
        Route::LogOut => html! { <LogOut /> },
        Route::NotFound => html! { <NotFound />},
        Route::VerificationFailed => html! { <VerificationFailed/> },
        Route::VerificationSucceeded => html! {<VerificationSucceeded />},
        Route::Contact => todo!(),
        Route::GDPR => todo!(),
        Route::Verify { token } => {
            html! {         <Suspense fallback={html!{<Loading/>}}><Verify {token} /></Suspense> }
        }
    }
}
