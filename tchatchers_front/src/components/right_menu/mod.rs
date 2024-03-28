// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

pub mod message_rmenu;
pub mod profile_rmenu;

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast};
use yew::{function_component, html, Component, Context, Html, Properties};
use yew_agent_latest::worker::{use_worker_subscription, UseWorkerSubscriptionHandle};
use yew_router::scope_ext::{LocationHandle, RouterScopeExt};

use rmenu_service::*;

use self::{message_rmenu::MessageRMenu, profile_rmenu::ProfileRMenu};

const JS_EVENTS_TO_CANCEL: [&str; 2] = ["click", "contextmenu"];

fn close_context_menu_function(
    bridge: UseWorkerSubscriptionHandle<rmenu_service::RMenuBus>,
) -> Function {
    let closure: Box<dyn FnMut(_)> =
        Box::new(move |_: web_sys::MouseEvent| bridge.send(RMenusBusEvents::CloseRMenu));

    let closure = Closure::wrap(closure);

    let js_value = closure.into_js_value();

    js_value.unchecked_into::<Function>()
}

#[derive(Properties, PartialEq, Clone)]
pub struct RightMenuHOCProps;

#[function_component(RightMenuHOC)]
pub fn rmenu_hoc(_props: &RightMenuHOCProps) -> Html {
    let bridge = use_worker_subscription::<RMenuBus>();

    html! { <RightMenu {bridge} /> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    bridge: UseWorkerSubscriptionHandle<RMenuBus>,
}

pub enum Msg {
    HandleBusMessage(RMenusBusEvents),
    RepositionMenu,
}

pub struct RightMenu {
    visible: bool,
    pos_x: i32,
    pos_y: i32,
    close_events: Vec<Function>,
    _location_handle: LocationHandle,
    content: Html,
    menu_repositionned: bool,
}

impl Component for RightMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx
            .link()
            .add_location_listener(
                ctx.link()
                    .callback(|_e| Msg::HandleBusMessage(RMenusBusEvents::CloseRMenu)),
            )
            .unwrap();
        Self {
            visible: false,
            pos_x: 0,
            pos_y: 0,
            close_events: Vec::new(),
            _location_handle: listener,
            content: Html::default(),
            menu_repositionned: false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.bridge.len() < ctx.props().bridge.len() {
            let Some(last_msg) = ctx.props().bridge.last().cloned() else {
                panic!("Unreachable");
            };
            let last_msg = (*last_msg).clone();
            ctx.link().send_message(Msg::HandleBusMessage(last_msg));

            true
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let clear_events = || {
            if !self.close_events.is_empty() {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();

                for close_event in self.close_events.iter() {
                    for event in JS_EVENTS_TO_CANCEL {
                        body.remove_event_listener_with_callback(event, close_event)
                            .unwrap();
                    }
                }
            }
        };
        match msg {
            Msg::HandleBusMessage(m) => match m {
                RMenusBusEvents::OpenRMenu(x, y, kind) => {
                    clear_events();
                    self.pos_x = x;
                    self.pos_y = y;

                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let body = document.body().unwrap();
                    let bridge = &ctx.props().bridge;

                    let close_event = close_context_menu_function(bridge.clone());

                    for event in JS_EVENTS_TO_CANCEL {
                        body.add_event_listener_with_callback(event, &close_event)
                            .unwrap();
                    }

                    self.content = match kind {
                        RMenuKind::MessageRMenu(props) => html! { <MessageRMenu ..props />},
                        RMenuKind::ProfileRMenu(props) => html! { <ProfileRMenu ..props /> },
                    };

                    self.visible = true;
                    self.close_events.push(close_event);
                }
                RMenusBusEvents::CloseRMenu => {
                    clear_events();
                    self.content = Html::default();
                    self.visible = false;
                    self.menu_repositionned = false;
                }
            },
            Msg::RepositionMenu => {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let rmenu = document
                    .get_element_by_id("rmenu")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlElement>()
                    .unwrap();

                let (rmenu_width, rmenu_height) = (rmenu.offset_width(), rmenu.offset_height());
                let (window_width, window_height) = (
                    window.inner_width().unwrap().as_f64().unwrap().round() as i32,
                    window.inner_height().unwrap().as_f64().unwrap().round() as i32,
                );

                if window_width < self.pos_x + rmenu_width {
                    self.pos_x = window_width - rmenu_width;
                }

                if window_height < self.pos_y + rmenu_height {
                    self.pos_y = window_height - rmenu_height;
                }

                self.menu_repositionned = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!("left: {}px; top : {}px;", self.pos_x, self.pos_y);
        if self.visible && !self.menu_repositionned {
            ctx.link().send_message(Msg::RepositionMenu);
        }
        html! {
            <div id="rmenu" hidden={!self.visible} {style} dir={"ltr"} class={"text-sm rounded flex-row-reverse z-1000 absolute bg-zinc-900 text-white drop-shadow-md p-2 dark:border-zinc-300 whitespace-nowrap"}>
                {self.content.clone()}
            </div>
        }
    }
}
