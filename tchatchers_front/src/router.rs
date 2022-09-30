use crate::components::feed::Feed;
use crate::components::join_room::JoinRoom;
use crate::components::logout::LogOut;
use crate::components::not_found::NotFound;
use crate::components::settings::Settings;
use crate::components::signin::SignIn;
use crate::components::signup::SignUp;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    JoinRoom,
    #[at("/r/:room")]
    Room { room: String },
    #[at("/signin")]
    SignIn,
    #[at("/signup")]
    SignUp,
    #[at("/settings")]
    Settings,
    #[at("/logout")]
    LogOut,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::JoinRoom => html! { <JoinRoom /> },
        Route::Room { room } => html! { <Feed room={room.clone()} /> },
        Route::SignIn => html! { <SignIn /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Settings => html! { <Settings /> },
        Route::LogOut => html! { <LogOut /> },
        Route::NotFound => html! { <NotFound />},
    }
}

impl Route {
    pub fn requires_auth(&self) -> bool {
        matches!(self, Route::Room { room: _ } | Route::Settings)
    }
}
