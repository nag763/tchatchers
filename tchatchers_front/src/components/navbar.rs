use std::{collections::HashMap, rc::Rc};

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use super::common::I18N;
use crate::{router::Route, utils::client_context::ClientContext};
use yew::{function_component, html, use_context, Component, Context, Html, Properties};
use yew_router::{prelude::Link, Routable};

#[function_component]
pub fn NavbarHOC() -> Html {
    let client_context =
        use_context::<Rc<ClientContext>>().expect("Client context defined at startup.");
    html! { <Navbar app_context={(*client_context).clone()}/> }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    app_context: ClientContext,
}

#[derive(Default, Debug, PartialEq)]
pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let client_context = ctx.props().app_context.clone();
        html! {
            <nav class="flex items-center justify-between flex-wrap bg-zinc-800 px-6 row-span-1">
                <Link<Route> to={if client_context.user.is_some() { Route::JoinRoom } else { Route::SignIn }} classes="flex items-center flex-shrink-0 text-white mr-6 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-600 hover:animate-pulse">
                    <img src="/favicon.ico" class="h-8 w-8 sm:h-12 sm:w-12 md:h-16 md:w-16"/>
                </Link<Route>>
                <div>
                    {
                        (*client_context.navlink).clone().into_iter().map(|n| html!{
                            <Link<Route> key={n.id} to={Route::from_path(&n.href, &HashMap::default()).unwrap()} classes="inline-block text-sm px-4 py-2 leading-none text-white" >
                                <I18N label={n.label} default={n.default_translation} translation={client_context.clone().translation}/>
                            </Link<Route> >
                        }).collect::<Html>()
                    }
                </div>
            </nav>
        }
    }
}
