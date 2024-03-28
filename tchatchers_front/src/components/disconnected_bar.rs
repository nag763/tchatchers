use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{WaitingForResponse, I18N};
use tchatchers_core::locale::TranslationMap;
use yew::{function_component, html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TryReconnectProps {
    translation: Rc<TranslationMap>,
}

#[function_component(TryReconnect)]
pub fn try_reconnect(props: &TryReconnectProps) -> Html {
    let translation = &props.translation;
    html! {
        <div class="flex items-center justify-center gap-2 lg:gap-12 dark:text-gray-200">
            <span>
            <I18N  label={"you_are_disconnected"} default={"You are disconnected"} {translation} />
            </span>
            <button class="common-button" onclick={move |_| {let _ = web_sys::window().unwrap().location().reload();}} >
            <I18N  label={"try_reconnect"} default={"Reconnect"} {translation} />
            </button>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub called_back: bool,
    pub translation: Rc<TranslationMap>,
}

pub struct DisconnectedBar;

impl Component for DisconnectedBar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = &ctx.props().translation;
        html! {
            <div class="col-span-6 mb-6">
                if ctx.props().called_back {
                    <TryReconnect {translation} />
                } else {
                    <WaitingForResponse {translation} />
                }
            </div>
        }
    }
}
