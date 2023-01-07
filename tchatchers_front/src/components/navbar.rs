// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use super::navlink::Navlink;
use crate::router::Route;
use tchatchers_core::app_context::AppContext;
use yew::{
    function_component, html, use_context, Component, Context, Html, Properties, UseStateHandle,
};
use yew_router::prelude::Link;

#[function_component]
pub fn NavbarHOC() -> Html {
    let app_context = use_context::<UseStateHandle<Option<AppContext>>>().expect("No app context");
    html! { <Navbar app_context={(*app_context).clone()}/> }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    app_context: Option<AppContext>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_user_logged_on = ctx.props().app_context.is_some();
        let links = match is_user_logged_on {
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
        let logo_route = match is_user_logged_on {
            true => Route::JoinRoom,
            false => Route::SignIn,
        };

        html! {
            <nav class="flex items-center justify-between flex-wrap bg-zinc-800 px-6 row-span-1">
                <Link<Route> to={logo_route} classes="flex items-center flex-shrink-0 text-white mr-6 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-600 hover:animate-pulse">
                    <img src="/favicon.ico" class="h-16 w-16"/>
                </Link<Route>>
                <div>
                    {links}
                </div>
            </nav>
        }
    }
}
