use super::chat::Chat;
use super::disconnected_bar::DisconnectedBar;
use super::type_bar::TypeBar;
use crate::router::Route;
use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use crate::services::message::*;
use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use linked_hash_set::LinkedHashSet;
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use wasm_bindgen::JsCast;
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;

const REFRESH_WS_STATE_EVERY: u32 = 5000;

#[derive(Clone)]
pub enum Msg {
    HandleMsg(String),
    CheckWsState,
    TryReconnect,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub room: String,
}

pub struct Feed {
    received_messages: LinkedHashSet<WsMessage>,
    ws: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
    _ws_reconnect: Option<Interval>,
    _first_connect: Timeout,
    called_back: bool,
    is_connected: bool,
    jwt: String,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ws: WebsocketService = WebsocketService::new();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let document_cookies = html_document.cookie().unwrap();
        let cookies = &mut document_cookies.split(";");
        let mut jwt_val: String = String::default();
        while let Some(cookie) = cookies.next() {
            if let Some(i) = cookie.find('=') {
                let (key, val) = cookie.split_at(i + 1);
                if key == "jwt=" {
                    jwt_val = val.into();
                }
            }
        }
        if jwt_val == String::default() {
            ctx.link().history().unwrap().push(Route::SignIn);
        }
        Self {
            received_messages: LinkedHashSet::new(),
            ws,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            is_connected: false,
            called_back: false,
            _ws_reconnect: None,
            _first_connect: {
                let link = ctx.link().clone();
                Timeout::new(1, move || link.send_message(Msg::CheckWsState))
            },
            jwt: jwt_val,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
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
                        self.received_messages
                            .insert(serde_json::from_str(&message.content).unwrap());
                        self.is_connected = true;
                    }
                    WsBusMessageType::Pong => {
                        self.is_connected = true;

                        let msg = WsMessage {
                            jwt: Some(self.jwt.clone()),
                            message_type: WsMessageType::RetrieveMessages,
                            room: Some(ctx.props().room.clone()),
                            ..WsMessage::default()
                        };
                        self.ws
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&msg).unwrap())
                            .unwrap();
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let component: Html = match self.is_connected {
            true => {
                let tx = self.ws.tx.clone();
                let pass_message_to_ws = Callback::from(move |message: String| {
                    tx.clone().try_send(message).unwrap();
                });
                html! {<TypeBar {pass_message_to_ws} jwt={self.jwt.clone()} room={ctx.props().room.clone()}/>}
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
            <div class="grid grid-rows-11 h-full">
                <div class="row-span-10" >
                    <Chat messages={self.received_messages.clone()} room={ctx.props().room.clone()} />
                </div>
                <div class="row-span-1 grid grid-cols-6 px-5 gap-4 justify-center content-center border-y-2 bg-slate-100 shadow-xl block">
                    {component}
                </div>
            </div>
        }
    }
}
