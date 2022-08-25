use super::navlink::Navlink;
use crate::router::Route;
use yew::{html, Component, Context, Html, Properties};
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="flex items-center justify-between flex-wrap bg-gray-800 px-6 row-span-1">
                <Link<Route> to={Route::Home} classes="flex items-center flex-shrink-0 text-white mr-6 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-600 hover:animate-pulse">
                    <span class="font-semibold text-xl tracking-tight">{ "kikers" }</span>
                </Link<Route>>
                <div>
                    <Navlink label="Settings" link={Route::Settings} />
                </div>
            </nav>
        }
    }
}
