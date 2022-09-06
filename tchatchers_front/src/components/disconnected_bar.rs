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
        <div class="flex items-center justify-center gap-2 lg:gap-12">
            <span>
            {"You are currently disconnected"}
            </span>
            <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={onclick} >
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
