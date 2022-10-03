use super::chat::Chat;
use super::disconnected_bar::DisconnectedBar;
use super::type_bar::TypeBar;
use crate::router::Route;
use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use crate::services::message::*;
use crate::utils::jwt::get_user;
use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::history::History;
use yew_router::prelude::HistoryListener;
use yew_router::scope_ext::RouterScopeExt;

const REFRESH_WS_STATE_EVERY: u32 = 5000;

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
    _ws_reconnect: Option<Interval>,
    _first_connect: Timeout,
    called_back: bool,
    is_connected: bool,
    user: PartialUser,
    _history_listener: HistoryListener,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ws: WebsocketService = WebsocketService::new(&ctx.props().room);
        let link = ctx.link().clone();
        let user = match get_user() {
            Ok(v) => v,
            Err(e) => {
                gloo_console::log!("Error while attempting to get the user :", e);
                link.history().unwrap().push(Route::SignIn);
                PartialUser::default()
            }
        };
        Self {
            received_messages: vec![],
            ws,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleWsInteraction)),
            is_connected: false,
            called_back: false,
            _ws_reconnect: None,
            user,
            _first_connect: {
                let link = ctx.link().clone();
                Timeout::new(1, move || link.send_message(Msg::CheckWsState))
            },
            _history_listener: ctx
                .link()
                .history()
                .unwrap()
                .listen(move || link.send_message(Msg::CutWs)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleWsInteraction(s) => {
                self.called_back = true;
                let message: WsBusMessage = serde_json::from_str(&s).unwrap();
                match message.message_type {
                    WsBusMessageType::NotConnected | WsBusMessageType::Closed => {
                        gloo_console::error!("Not connected");
                        let req = Request::get("/api/validate").send();
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.await.unwrap();
                            if resp.status() == 401 {
                                link.history().unwrap().push(Route::SignIn);
                            }
                        });
                        let link = ctx.link().clone();
                        self.is_connected = false;
                        self._ws_reconnect = Some({
                            Interval::new(REFRESH_WS_STATE_EVERY, move || {
                                link.send_message(Msg::CheckWsState)
                            })
                        });
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
                gloo_console::debug!("Try reconnect by user");
                self.called_back = false;
                self.ws.tx.clone().try_send("Ping".into()).unwrap();
                true
            }
            Msg::CutWs => {
                self.ws.tx.clone().try_send("Close".into()).unwrap();
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
