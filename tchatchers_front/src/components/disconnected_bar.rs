use crate::components::common::WaitingForResponse;
use yew::{function_component, html, Callback, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TryReconnectProps {
    try_reconnect: Callback<()>,
}

#[function_component(TryReconnect)]
pub fn try_reconnect(props: &TryReconnectProps) -> Html {
    let onclick_event = props.try_reconnect.clone();
    let onclick = move |_| {
        onclick_event.emit(());
    };
    html! {
        <div class="flex items-center justify-center gap-2 lg:gap-12 dark:text-gray-200">
            <span>
            {"You are currently disconnected"}
            </span>
            <button class="shadow bg-zinc-800 dark:bg-gray-500 enabled:hover:bg-zinc-900 dark:enabled:hover:bg-gray-600 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded-md" onclick={onclick} >
            {"Reconnect"}
            </button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub called_back: bool,
    pub try_reconnect: Callback<()>,
}

pub struct DisconnectedBar;

impl Component for DisconnectedBar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let component = match ctx.props().called_back {
            true => html! { <TryReconnect try_reconnect={ctx.props().try_reconnect.clone()} /> },
            false => html! { <WaitingForResponse /> },
        };
        html! {
            <div class="col-span-6">
                {component}
            </div>
        }
    }
}
