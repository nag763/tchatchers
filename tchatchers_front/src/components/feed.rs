use super::chat::Chat;
use super::disconnected_bar::DisconnectedBar;
use super::type_bar::TypeBar;
use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use crate::services::message::*;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use yew::{html, Component, Context, Html, Properties, Callback};
use yew_agent::{Bridge, Bridged};

const REFRESH_WS_STATE_EVERY: u32 = 5000;

#[derive(Clone)]
pub enum Msg {
    HandleMsg(String),
    CheckWsState,
    TryReconnect,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Feed {
    received_messages: Vec<String>,
    ws: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
    _ws_reconnect: Option<Interval>,
    _first_connect: Timeout,
    called_back: bool,
    is_connected: bool,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ws: WebsocketService = WebsocketService::new();
        Self {
            received_messages: vec![],
            ws,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            is_connected: false,
            called_back: false,
            _ws_reconnect: None,
            _first_connect: {
                let link = ctx.link().clone();
                Timeout::new(1, move || link.send_message(Msg::CheckWsState))
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                self.called_back = true;
                let message: WsMessage = serde_json::from_str(&s).unwrap();
                match message.message_type {
                    WsMessageType::NotConnected | WsMessageType::Closed => {
                        gloo_console::error!("Not connected");
                        self.is_connected = false;
                        self._ws_reconnect = Some({
                            let link = _ctx.link().clone();
                            Interval::new(REFRESH_WS_STATE_EVERY, move || {
                                link.send_message(Msg::CheckWsState)
                            })
                        });
                    }
                    WsMessageType::Reply => {
                        self.received_messages.push(message.content);
                        self.is_connected = true;
                    }
                    WsMessageType::Pong => {
                        self.is_connected = true;
                    }
                    _ => {
                        self.is_connected = true;
                    }
                }
                gloo_console::log!("Is connected :", self.is_connected);
                true
            }
            Msg::CheckWsState => {
                gloo_console::log!("Checkin ws");
                self.ws.tx.clone().try_send("Ping".into()).unwrap();
                false
            }
            Msg::TryReconnect => {
                gloo_console::log!("Try reconnect by user");
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
                html! {<TypeBar {pass_message_to_ws}/>} 
            },
            false => {
                let link = ctx.link().clone();
                let try_reconnect = Callback::from(move |_: ()| {
                    gloo_console::log!("Hey I am here!");
                    link.send_message(Msg::TryReconnect);
                });
                html! {<DisconnectedBar called_back={self.called_back} {try_reconnect} />}
            }
        };
        html! {
            <div class="grid grid-rows-11 h-full">
                <div class="row-span-10" >
                    <Chat messages={self.received_messages.clone()} />
                </div>
                <div class="row-span-1 grid grid-cols-6 px-5 gap-4 justify-center content-center border-y-2 bg-slate-100 shadow-xl block">
                    {component}
                </div>
            </div>
        }
    }
}
