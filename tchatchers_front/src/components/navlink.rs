use crate::router::Route;
use yew::{html, Component, Context, Html, Properties};
use yew_router::prelude::Link;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
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
                {ctx.props().clone().label}
                </Link<Route> >
        }
    }
}
