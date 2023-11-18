// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use gloo_timers::callback::Timeout;
use modal_service::{ModalBus, ModalBusContent, ModalContent};
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, EventTarget, MouseEvent};
use yew::{classes, html, Component, Context, Html};
use yew_agent::{Bridge, Bridged};

const MODAL_ID: &str = "modal";
pub const MODAL_OPENER_CLASS: &str = "modal-opener";

pub enum Msg {
    CloseSelf(Option<ModalBusContent>),
    PopModal(ModalContent),
    StopBounce,
}

pub struct Modal {
    modal_content: ModalContent,
    visible: bool,
    producer: Box<dyn Bridge<ModalBus>>,
    stop_bounce: Option<Timeout>,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |mc| {
                if let ModalBusContent::PopModal(mbc) = mc {
                    link.send_message(Msg::PopModal(mbc))
                }
            }
        };
        Self {
            modal_content: ModalContent::default(),
            visible: false,
            producer: ModalBus::bridge(Rc::new(cb)),
            stop_bounce: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseSelf(mbc) => {
                if let Some(mbc) = mbc {
                    self.producer.send(mbc);
                }
                self.visible = false;
                true
            }
            Msg::PopModal(modal_content) => {
                self.stop_bounce = Some({
                    let link = ctx.link().clone();
                    Timeout::new(500, move || link.send_message(Msg::StopBounce))
                });
                self.modal_content = modal_content;
                self.visible = true;
                true
            }
            Msg::StopBounce => {
                self.stop_bounce = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let decline_text = match &self.modal_content.decline_text {
            Some(v) => v,
            None => "Cancel",
        };
        let accept_text = match &self.modal_content.accept_text {
            Some(v) => v,
            None => "Ok",
        };
        let link = ctx.link().clone();
        let decline_callback = move |_: MouseEvent| {
            link.send_message(Msg::CloseSelf(Some(ModalBusContent::Outcome(false))))
        };
        let link = ctx.link().clone();
        let accept_callback = move |_: MouseEvent| {
            link.send_message(Msg::CloseSelf(Some(ModalBusContent::Outcome(true))));
        };

        let link = ctx.link().clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |me: web_sys::MouseEvent| {
            let target: Option<EventTarget> = me.target();
            let element = target.and_then(|t| t.dyn_into::<Element>().ok());
            if let Some(element) = element {
                if let Ok(maybe_undefined_element) =
                    element.closest(&format!("#{},.{}", MODAL_ID, MODAL_OPENER_CLASS))
                {
                    if maybe_undefined_element.is_none() {
                        link.send_message(Msg::CloseSelf(None));
                    }
                }
            }
        });
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .body()
            .unwrap()
            .set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        html! {
                <div id={MODAL_ID} hidden={!self.visible} class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-full max-w-2xl h-auto">
                    <div class={classes!("default-modal", self.stop_bounce.as_ref().map(|_| "animate-bounce"))}>
                        <div class="flex justify-between items-start p-4 rounded-t border-b border-slate-400 dark:border-gray-600">
                            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                                {&self.modal_content.title}
                            </h3>
                            <button type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white" data-modal-toggle="defaultModal" onclick={ctx.link().callback(|_| Msg::CloseSelf(None))}>
                                <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                                <span class="sr-only">{"Close modal"}</span>
                            </button>
                        </div>
                        <div class="p-6 space-y-6">
                            <p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
                                {&self.modal_content.msg}
                            </p>
                        </div>
                        <div class="flex justify-end items-center p-6 space-x-2 rounded-b border-t border-gray-200 dark:border-gray-600">
                            <button data-modal-toggle="defaultModal" type="button" class="text-gray-500 bg-gray-300 hover:bg-gray-400 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg border border-gray-200 text-sm font-medium px-5 py-2.5 hover:text-gray-900 focus:z-10 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-500 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-600" onclick={decline_callback}>{decline_text}</button>
                            <button data-modal-toggle="defaultModal" type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={accept_callback}>{accept_text}</button>
                        </div>
                    </div>
                </div>
        }
    }
}
