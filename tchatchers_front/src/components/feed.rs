use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use super::chat::Chat;
use super::disconnected_bar::DisconnectedBar;
use super::type_bar::TypeBar;
use crate::router::Route;
use crate::utils;
use crate::utils::client_context::ClientContext;
use crate::utils::requester::Requester;
use chat_service::{ChatReactor, WebSocketReactorControl};
use gloo_timers::callback::{Interval, Timeout};
use tchatchers_core::room::RoomNameValidator;
use tchatchers_core::ws_message::{WsMessage, WsMessageContent, WsReceptionStatus};
use toast_service::{Alert, ToastBus};
use uuid::Uuid;
use validator::Validate;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, Properties,
    UseStateHandle,
};
use yew_agent::Dispatched;
use yew_agent_latest::reactor::{use_reactor_subscription, UseReactorSubscriptionHandle};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Properties, PartialEq, Clone)]
pub struct FeedHOCProps {
    pub room: AttrValue,
}

#[function_component(FeedHOC)]
pub fn feed_hoc(props: &FeedHOCProps) -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let reactor = use_reactor_subscription::<ChatReactor>();

    html! { <Feed room={props.room.clone()} {client_context}  {reactor} /> }
}

#[derive(Clone)]
pub enum Msg {
    CheckWsState,
    CutWs,
    Authenticate,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub room: AttrValue,
    pub client_context: Rc<ClientContext>,
    pub reactor: UseReactorSubscriptionHandle<ChatReactor>,
}

pub struct Feed {
    received_messages: Vec<WsMessageContent>,
    timeout: Option<Timeout>,
    called_back: bool,
    is_connected: bool,
    ws_keep_alive: Option<Interval>,
    is_closed: bool,
    session_id: Uuid,
    room_name_checked: bool,
    user_context: ClientContext,
    bearer: UseStateHandle<Option<String>>,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let reactor = &ctx.props().reactor;
        reactor.send(WebSocketReactorControl::Open(utils::get_ws_room_address(
            &ctx.props().room,
        )));

        Self {
            received_messages: vec![],
            is_connected: false,
            called_back: false,
            is_closed: false,
            ws_keep_alive: None,
            timeout: {
                let link = ctx.link().clone();
                Some(Timeout::new(1, move || {
                    link.send_message(Msg::CheckWsState)
                }))
            },
            session_id: Uuid::new_v4(),
            room_name_checked: false,
            user_context: ctx.props().client_context.as_ref().clone(),
            bearer: ctx.props().client_context.bearer.clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let reactor = &ctx.props().reactor;
        let old_reactor_length = old_props.reactor.len();
        if old_reactor_length < reactor.len() {
            let Some(last_msg) = reactor.last().cloned() else {
                panic!("Unreachable");
            };
            let message: WsMessage = (*last_msg).clone();
            self.called_back = true;
            match message {
                WsMessage::ClientDisconnected | WsMessage::ConnectionClosed => {
                    if !self.is_closed {
                        gloo_console::error!("Not connected");
                        let mut req = Requester::get("/api/validate");
                        req.bearer(self.bearer.clone());
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.send().await;
                            if resp.status() == 401 {
                                link.navigator().unwrap().push(&Route::SignIn);
                            }
                        });
                        if !self.room_name_checked {
                            if let Err(_e) =
                                RoomNameValidator::from(ctx.props().room.to_string()).validate()
                            {
                                ToastBus::dispatcher().send(Alert {
                                        is_success: false,
                                        label: "room_name_incorrect".into(),
                                        default: "The room name you tried to join is not valid, please select one within this screen.".into(),
                                    });
                                ctx.link().navigator().unwrap().push(&Route::JoinRoom);
                            } else {
                                self.room_name_checked = true;
                            }
                        }
                        self.ws_keep_alive = None;
                        self.is_connected = false;
                    }
                }
                WsMessage::Receive(msg_content) => {
                    self.received_messages.insert(0, msg_content.clone());
                    if msg_content.reception_status == WsReceptionStatus::Sent
                        && msg_content.author.id != self.user_context.user.as_ref().unwrap().id
                    {
                        reactor.send(WebSocketReactorControl::Send(WsMessage::Seen(vec![
                            msg_content.uuid,
                        ])));
                    }
                }
                WsMessage::MessagesRetrieved {
                    mut messages,
                    session_id,
                } if session_id == self.session_id => {
                    let messages_seen: Vec<Uuid> = messages
                        .clone()
                        .into_iter()
                        .filter(|message| {
                            message.reception_status == WsReceptionStatus::Sent
                                && message.author.id != self.user_context.user.as_ref().unwrap().id
                        })
                        .map(|m| m.uuid)
                        .collect();
                    self.received_messages.append(&mut messages);

                    if !messages_seen.is_empty() {
                        reactor.send(WebSocketReactorControl::Send(WsMessage::Seen(
                            messages_seen,
                        )));
                    }
                }
                WsMessage::Pong => {
                    self.is_connected = true;

                    ctx.link().send_message(Msg::Authenticate);
                }
                WsMessage::MessagesSeen(msgs_uuid) => {
                    for msg in self.received_messages.iter_mut() {
                        if msgs_uuid.contains(&msg.uuid) {
                            msg.reception_status = WsReceptionStatus::Seen;
                        }
                    }
                }
                WsMessage::Delete(msg_uuid) => {
                    reactor.send(WebSocketReactorControl::Send(message));
                    self.received_messages.retain(|msg| msg_uuid != msg.uuid);
                }
                WsMessage::AuthenticationRequired => ctx.link().send_message(Msg::Authenticate),
                WsMessage::AuthenticationValidated => {
                    if self.received_messages.is_empty() {
                        let msg = WsMessage::RetrieveMessages(self.session_id);

                        reactor.send(WebSocketReactorControl::Send(msg));
                        self.ws_keep_alive = {
                            let reactor = reactor.clone();
                            Some(Interval::new(30_000, move || {
                                reactor.send(WebSocketReactorControl::Send(
                                    WsMessage::ClientKeepAlive,
                                ));
                            }))
                        }
                    }
                }
                WsMessage::AuthenticationExpired => {
                    let mut req = Requester::get("/api/validate");
                    req.bearer(self.bearer.clone());
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let resp = req.send().await;
                        if resp.status() == 401 {
                            link.navigator().unwrap().push(&Route::SignIn);
                        } else {
                            link.send_message(Msg::Authenticate);
                        }
                    });
                }
                _ => {
                    self.is_connected = true;
                }
            }

            true
        // Other props can't change
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let reactor = &ctx.props().reactor;
        match msg {
            Msg::CheckWsState => {
                reactor.send(WebSocketReactorControl::Send(WsMessage::Ping));
                if self.timeout.is_some() {
                    self.timeout = None;
                }
                false
            }
            Msg::CutWs => {
                self.is_closed = true;
                reactor.send(WebSocketReactorControl::Close);
                true
            }
            Msg::Authenticate => {
                if let Some(bearer) = self.bearer.as_ref().cloned() {
                    reactor.send(WebSocketReactorControl::Send(WsMessage::Authenticate(
                        bearer,
                    )));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = &ctx.props().client_context.translation;
        let reactor = &ctx.props().reactor;
        html! {
            <div class="grid grid-rows-11 auto-rows-fr h-full dark:bg-zinc-800">
                <div class="row-span-10 overflow-auto flex flex-col-reverse max-h-full mt-4" >
                    <Chat messages={self.received_messages.clone()} room={ctx.props().room.clone()} user={self.user_context.user.as_ref().unwrap().clone()} />
                </div>
                <div class="row-span-1 grid grid-cols-6 px-5 gap-4 justify-center content-center block ">
                    if self.is_connected {
                        <TypeBar {translation} pass_message_to_ws={{let reactor = reactor.clone(); move |message| reactor.send(WebSocketReactorControl::Send(message))}} user={self.user_context.user.as_ref().unwrap().clone()} room={ctx.props().room.clone()} />
                    } else {
                        <DisconnectedBar {translation} called_back={self.called_back}  />
                    }
                </div>
            </div>
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        let reactor = &ctx.props().reactor;
        reactor.send(WebSocketReactorControl::Close);
    }
}
