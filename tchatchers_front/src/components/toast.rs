use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::{services::toast_bus::ToastBus, utils::client_context::ClientContext};
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, use_context, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};

#[function_component(ToastHOC)]
pub fn sign_up_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().expect("No app context");
    html! { <Toast client_context={(*client_context).clone()}/> }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Alert {
    pub is_success: bool,
    pub label: String,
    pub default: String,
}

pub enum Msg {
    NewToast(Alert),
    Hide,
    StopBounce,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    client_context: ClientContext,
}

pub struct Toast {
    msg: String,
    is_success: bool,
    visible: bool,
    cooldown: Option<Timeout>,
    stop_bounce: Option<Timeout>,
    _producer: Box<dyn Bridge<ToastBus>>,
}

impl Component for Toast {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |e| link.send_message(Msg::NewToast(e))
        };
        Self {
            msg: String::default(),
            is_success: false,
            visible: false,
            cooldown: None,
            stop_bounce: None,
            _producer: ToastBus::bridge(Rc::new(cb)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewToast(alert) => {
                self.msg = ctx
                    .props()
                    .client_context
                    .translation
                    .get_or_default(&alert.label, "Message unknown");
                self.is_success = alert.is_success;
                self.visible = true;
                self.cooldown = Some({
                    let link = ctx.link().clone();
                    Timeout::new(3_000, move || link.send_message(Msg::Hide))
                });
                self.stop_bounce = Some({
                    let link = ctx.link().clone();
                    Timeout::new(500, move || link.send_message(Msg::StopBounce))
                });
            }
            Msg::Hide => {
                self.visible = false;
            }
            Msg::StopBounce => {
                self.stop_bounce = None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let icon = match self.is_success {
            true => html! {
            <div class="inline-flex flex-shrink-0 justify-center items-center w-8 h-8 text-green-500 bg-green-100 rounded-lg dark:bg-green-800 dark:text-green-200">
                <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>
                <span class="sr-only">{"Check icon"}</span>
            </div>
                    },
            false => html! {
            <div class="inline-flex flex-shrink-0 justify-center items-center w-8 h-8 text-red-500 bg-red-100 rounded-lg dark:bg-red-800 dark:text-red-200">
                <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                <span class="sr-only">{"Error icon"}</span>
            </div>

                    },
        };
        let bouncer: &str = match self.stop_bounce {
            Some(_) => "animate-bounce",
            None => "",
        };
        html! {
                    <div class={format!("z-50 absolute right-3 bottom-5 md:bottom-10 md:right-10 {}", bouncer)} hidden={!self.visible}>
            <div id="toast-success" class="flex items-center p-4 mb-4 w-full max-w-xs text-gray-500 bg-white rounded-lg shadow dark:text-gray-400 dark:bg-zinc-900" role="alert">
                {icon}
            <div class="ml-3 text-sm font-normal">{self.msg.as_str()}</div>
            <button type="button" class="ml-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 rounded-lg focus:ring-2 focus:ring-gray-300 focus:ring-zinc-800 p-1.5 hover:bg-gray-100 inline-flex h-8 w-8 dark:text-gray-500 dark:hover:text-white dark:bg-zinc-800 dark:hover:bg-gray-700 ml-4" data-dismiss-target="#toast-success" aria-label="Close" onclick={ctx.link().callback(|_| Msg::Hide)}>
                <span class="sr-only">{"Close"}</span>
                <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
            </button>
        </div>
            </div>
                }
    }
}
