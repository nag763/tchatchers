use crate::services::message::*;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, Properties};

use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use yew::NodeRef;
use yew_agent::{Bridge, Bridged};

const PROGRESS_REFRESH: u32 = 20;
const REFRESH_WS_STATE_EVERY: u32 = 5000;
const TIMEOUT: u32 = 15_000;

pub enum Msg {
    HandleMsg(String),
    SubmitForm,
    InputChanged,
    FileAttached,
    Reactivate,
    UpdateProgress,
    CheckWsState,
    TryReconnect,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Feed {
    text: String,
    file: Option<String>,
    input_ref: NodeRef,
    attach_ref: NodeRef,
    placeholder_input: String,
    cooldown: Option<Timeout>,
    progress: Option<Interval>,
    progress_percentage: u32,
    error_on_input: bool,
    can_post: bool,
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
            attach_ref: NodeRef::default(),
            cooldown: None,
            progress: None,
            error_on_input: false,
            file: None,
            input_ref: NodeRef::default(),
            progress_percentage: 0,
            text: String::new(),
            received_messages: vec![],
            ws,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            can_post: true,
            placeholder_input: String::from("Type a message"),
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
                gloo_console::log!("User tried");
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
                    },
                    _ => {
                        self.is_connected = true;
                    }
                }
                gloo_console::log!("Is connected :", self.is_connected);
                true
            }
            Msg::SubmitForm => {
                if 2 < self.text.len() && self.text.len() < 128 {
                    self.text = String::new();
                    if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                        self.ws.tx.clone().try_send(input.value()).unwrap();
                        input.set_value("");
                    }
                    if let Some(input) = self.attach_ref.cast::<HtmlInputElement>() {
                        input.set_value("");
                    }
                    self.file = None;
                    self.can_post = false;
                    self.error_on_input = false;
                    self.placeholder_input = String::from("Wait few seconds before typing again");
                    self.cooldown = Some({
                        let link = _ctx.link().clone();
                        Timeout::new(TIMEOUT, move || link.send_message(Msg::Reactivate))
                    });
                    self.progress = Some({
                        let link = _ctx.link().clone();
                        Interval::new(PROGRESS_REFRESH, move || {
                            link.clone().send_message(Msg::UpdateProgress);
                        })
                    });
                    true
                } else {
                    self.error_on_input = true;
                    true
                }
            }
            Msg::UpdateProgress => {
                self.progress_percentage += PROGRESS_REFRESH;
                true
            }
            Msg::InputChanged => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    self.text = input.value();
                    self.error_on_input = false;
                    true
                } else {
                    false
                }
            }
            Msg::FileAttached => {
                if let Some(input) = self.attach_ref.cast::<HtmlInputElement>() {
                    self.file = Some(input.value());
                    true
                } else {
                    false
                }
            }
            Msg::Reactivate => {
                self.can_post = true;
                self.placeholder_input = String::from("Type a message");
                self.cooldown = None;
                self.progress = None;
                self.progress_percentage = 0;
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
        html! {
            <div class="grid grid-rows-11 h-full">
            <div class="row-span-10" >
                {self.received_messages.iter().map(|m| html!{<p>{m}</p>}).collect::<Html>()}
            </div>
            <div class="row-span-1 grid grid-cols-6 px-5 gap-4 justify-center content-center border-y-2 bg-slate-100 shadow-xl block">
                    <div class="grid justify-items-center content-center">
                    <label hidden={self.file.is_some() || !self.is_connected} for="file-upload">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
        </svg>
                    </label>
                    <label hidden={self.file.is_none()} for="file-upload">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
        </svg>
                    </label>
                        <input id="file-upload" type="file" ref={self.attach_ref.clone()} style="display: none;" disabled={!self.can_post || !self.is_connected} oninput={ctx.link().callback(|_| Msg::FileAttached)}/>

                    </div>
                    <div class="col-span-4">
                    <div hidden={self.is_connected}>
                    <div hidden={!self.called_back}>
                        <div class="flex items-center justify-center gap-2 lg:gap-12"><span>{"You are currently disconnected"}</span>
                      <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::TryReconnect)}>

                      {"Reconnect"}
                      </button>
                          </div>
                          </div>
                          <div hidden={self.called_back}>
                            <p class="flex justify-center">{"Waiting for server reply"}
                          <svg class="inline ml-2 w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
        <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
    </svg>

                      </p>
                          </div>
                          </div>
                      <input class="shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-indigo-500 w-full h-10 invalid:border-red-500 disabled:bg-gray-100" id="username" type="text" placeholder={self.placeholder_input.clone()} minlength="3" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post || !self.is_connected} oninput={ctx.link().callback(|_| Msg::InputChanged)} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::InputChanged }})} hidden={!self.is_connected} />
                  <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
        <div class="bg-gradient-to-r from-indigo-300 to-indigo-600 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}></div>
        </div>
                      <div class="col-span-4" hidden={!self.error_on_input} >
                      <small class="text-red-500">{"Your message is either too short, or too long, ensure it matches the requirements"}</small>
                      </div>
                      </div>
                      <div class="flex justify-center">
                      <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::SubmitForm)} disabled={!self.can_post || !self.is_connected}  hidden={!self.is_connected} >

                      {"Post"}
                      </button>
                      </div>
                      <div class="col-span-1">
                      </div>
                  </div>
            </div>
        }
    }
}
