use crate::components::feed::Feed;
use crate::components::settings::Settings;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    Settings,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Feed /> },
        Route::Settings => html! { <Settings /> },
    }
}
