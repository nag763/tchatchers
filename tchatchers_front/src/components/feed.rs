// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use super::chat::Chat;
use super::disconnected_bar::DisconnectedBar;
use super::type_bar::TypeBar;
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use crate::services::message::*;
use crate::services::toast_bus::ToastBus;
use crate::utils::jwt::get_user;
use gloo_net::http::Request;
use gloo_timers::callback::{Interval, Timeout};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_agent::Dispatched;
use yew_agent::{Bridge, Bridged};
use yew_router::history::History;
use yew_router::prelude::HistoryListener;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone)]
pub enum Msg {
    HandleWsInteraction(String),
    CheckWsState,
    TryReconnect,
    CutWs,
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub room: String,
}

pub struct Feed {
    received_messages: Vec<WsMessage>,
    ws: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
    _first_connect: Timeout,
    called_back: bool,
    is_connected: bool,
    ws_keep_alive: Option<Interval>,
    is_closed: bool,
    user: PartialUser,
    _history_listener: HistoryListener,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ws: WebsocketService = WebsocketService::new(&ctx.props().room);
        let link = ctx.link().clone();
        let tx = ws.tx.clone();
        let user = match get_user() {
            Ok(v) => v,
            Err(e) => {
                gloo_console::log!("Error while attempting to get the user :", e);
                link.history().unwrap().push(Route::SignIn);
                ToastBus::dispatcher().send(Alert {
                    is_success: false,
                    content: "Please authenticate prior accessing the app functionnalities.".into(),
                });
                PartialUser::default()
            }
        };
        Self {
            received_messages: vec![],
            ws,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleWsInteraction)),
            is_connected: false,
            called_back: false,
            is_closed: false,
            ws_keep_alive: None,
            user,
            _first_connect: {
                let link = ctx.link().clone();
                Timeout::new(1, move || link.send_message(Msg::CheckWsState))
            },
            _history_listener: ctx.link().history().unwrap().listen(move || {
                let mut tx = tx.clone();
                tx.try_send("Close".into()).unwrap();
                link.send_message(Msg::CutWs)
            }),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleWsInteraction(s) => {
                self.called_back = true;
                let message: WsBusMessage = serde_json::from_str(&s).unwrap();
                match message.message_type {
                    WsBusMessageType::NotConnected | WsBusMessageType::Closed => {
                        if !self.is_closed {
                            gloo_console::error!("Not connected");
                            let req = Request::get("/api/validate").send();
                            let link = ctx.link().clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let resp = req.await.unwrap();
                                if resp.status() == 401 {
                                    link.history().unwrap().push(Route::SignIn);
                                }
                            });
                            self.is_connected = false;
                        }
                    }
                    WsBusMessageType::Reply => {
                        let msg: WsMessage = serde_json::from_str(&message.content).unwrap();
                        match msg.to {
                            Some(v) if v != self.user => {
                                return true;
                            }
                            _ => {
                                self.received_messages.insert(0, msg);
                            }
                        }
                        self.is_connected = true;
                        self.ws_keep_alive = {
                            let tx = self.ws.tx.clone();
                            Some(Interval::new(30_000, move || {
                                tx.clone().try_send("Keep Alive".into()).unwrap()
                            }))
                        }
                    }
                    WsBusMessageType::Pong => {
                        self.is_connected = true;

                        if self.received_messages.is_empty() {
                            let msg = WsMessage {
                                message_type: WsMessageType::RetrieveMessages,
                                author: Some(self.user.clone()),
                                room: Some(ctx.props().room.clone()),
                                ..WsMessage::default()
                            };
                            self.ws
                                .tx
                                .clone()
                                .try_send(serde_json::to_string(&msg).unwrap())
                                .unwrap();
                        }
                    }
                    _ => {
                        self.is_connected = true;
                    }
                }
                true
            }
            Msg::CheckWsState => {
                self.ws.tx.clone().try_send("Ping".into()).unwrap();
                false
            }
            Msg::TryReconnect => {
                let ws: WebsocketService = WebsocketService::new(&ctx.props().room);
                self.ws = ws;
                self.ws.tx.clone().try_send("Ping".into()).unwrap();
                self.called_back = false;
                true
            }
            Msg::CutWs => {
                self.is_closed = true;
                let mut ws = self.ws.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    ws.close().await;
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let component: Html = match self.is_connected {
            true => {
                let tx = self.ws.tx.clone();
                let pass_message_to_ws = Callback::from(move |message: String| {
                    tx.clone().try_send(message).unwrap();
                });
                html! {<TypeBar {pass_message_to_ws} user={self.user.clone()} room={ctx.props().room.clone()}/>}
            }
            false => {
                let link = ctx.link().clone();
                let try_reconnect = Callback::from(move |_: ()| {
                    link.send_message(Msg::TryReconnect);
                });
                html! {<DisconnectedBar called_back={self.called_back} {try_reconnect} />}
            }
        };
        html! {
            <div class="grid grid-rows-11 h-full dark:bg-zinc-800">
                <div class="row-span-10 overflow-auto flex flex-col-reverse" >
                    <Chat messages={self.received_messages.clone()} room={ctx.props().room.clone()} user={self.user.clone()} />
                </div>
                <div class="row-span-1 grid grid-cols-6 px-5 gap-4 justify-center content-center block">
                    {component}
                </div>
            </div>
        }
    }
}
