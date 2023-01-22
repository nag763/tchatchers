// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::router::Route;
use yew::{html, AttrValue, Component, Context, Html, Properties};
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub label: AttrValue,
    pub link: Route,
}

pub struct Navlink;

impl Component for Navlink {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Link<Route> to={ctx.props().clone().link} classes="inline-block text-sm px-4 py-2 leading-none text-white" >
                {ctx.props().label.as_str()}
            </Link<Route> >
        }
    }
}
