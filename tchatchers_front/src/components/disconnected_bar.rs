use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{WaitingForResponse, I18N};
use tchatchers_core::locale::Translation;
use yew::{function_component, html, Callback, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TryReconnectProps {
    try_reconnect: Callback<()>,
    translation: Rc<Translation>,
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
            <I18N  label={"you_are_disconnected"} default={"You are disconnected"} translation={props.translation.clone()}/>
            </span>
            <button class="shadow bg-zinc-800 dark:bg-gray-500 enabled:hover:bg-zinc-900 dark:enabled:hover:bg-gray-600 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded-md" onclick={onclick} >
            <I18N  label={"try_reconnect"} default={"Reconnect"} translation={props.translation.clone()}/>
            </button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub called_back: bool,
    pub try_reconnect: Callback<()>,
    pub translation: Rc<Translation>,
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
            true => {
                html! { <TryReconnect translation={ctx.props().translation.clone()} try_reconnect={ctx.props().try_reconnect.clone()} /> }
            }
            false => html! { <WaitingForResponse /> },
        };
        html! {
            <div class="col-span-6">
                {component}
            </div>
        }
    }
}
