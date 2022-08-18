use yew::{Component, Html, html, Context, Properties};
use super::navlink::Navlink;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="flex items-center justify-between flex-wrap bg-gray-800 px-6 lg:py-6 row-span-1 lg:row-auto">
                <a href="/" class="flex items-center flex-shrink-0 text-white mr-6 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-600 hover:animate-pulse">
	                <span class="font-semibold text-xl tracking-tight">{ "kikers" }</span>
                </a>
                <div>
                    <Navlink label="My label" link="My link" />
                    <Navlink label="My label" link="My link" />
                </div>
            </nav>
        }
    }
}
