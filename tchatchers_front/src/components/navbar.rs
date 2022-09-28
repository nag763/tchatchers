use super::navlink::Navlink;
use crate::router::Route;
use crate::services::auth_bus::EventBus;
use yew::{html, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::Link;

pub enum Msg {
    AuthEvent(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props;

pub struct Navbar {
    verified: bool,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            verified: false,
            _producer: EventBus::bridge(ctx.link().callback(Msg::AuthEvent)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AuthEvent(e) => {
                self.verified = e;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let links = match self.verified {
            true => html! {
                <>
                    <Navlink label="Settings" link={Route::Settings} />
                    <Navlink label="Log out" link={Route::LogOut} />
                </>
            },
            false => html! {
                <>
                    <Navlink label="Sign in" link={Route::SignIn} />
                    <Navlink label="Sign up" link={Route::SignUp} />
                </>
            },
        };
        let logo_route = match self.verified {
            true => Route::Home,
            false => Route::SignIn,
        };

        html! {
            <nav class="flex items-center justify-between flex-wrap bg-gray-800 px-6 row-span-1">
                <Link<Route> to={logo_route} classes="flex items-center flex-shrink-0 text-white mr-6 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-600 hover:animate-pulse">
                    <span class="font-semibold text-xl tracking-tight">{ "tchatchers" }</span>
                </Link<Route>>
                <div>
                    {links}
                </div>
            </nav>
        }
    }
}
