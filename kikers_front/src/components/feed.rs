use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, Properties};

use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use gloo_timers::callback::Interval;
use gloo_timers::callback::Timeout;
use yew::NodeRef;
use yew_agent::{Bridge, Bridged};

const PROGRESS_REFRESH: u32 = 20;
const TIMEOUT: u32 = 15_000;

pub enum Msg {
    HandleMsg(String),
    SubmitForm,
    InputChanged,
    FileAttached,
    Reactivate,
    UpdateProgress,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Feed {
    pub text: String,
    pub file: Option<String>,
    pub input_ref: NodeRef,
    pub attach_ref: NodeRef,
    placeholder_input: String,
    cooldown: Option<Timeout>,
    progress: Option<Interval>,
    progress_percentage: u32,
    error_on_input: bool,
    can_post: bool,
    received_messages: Vec<String>,
    ws: WebsocketService,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ws = WebsocketService::new();
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                self.received_messages.push(s);
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="grid grid-cols-6 py-5 px-5 gap-4 justify-center">
                    <div class="grid justify-items-center content-center">
                    <label hidden={self.file.is_some()} for="file-upload">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
        </svg>
                    </label>
                    <label hidden={self.file.is_none()} for="file-upload">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
        </svg>
                    </label>
                        <input id="file-upload" type="file" ref={self.attach_ref.clone()} style="display: none;" oninput={ctx.link().callback(|_| Msg::FileAttached)}/>

                    </div>
                    <div class="col-span-4">
                      <input class="shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-indigo-500 w-full h-10 invalid:border-red-500 disabled:bg-gray-100" id="username" type="text" placeholder={self.placeholder_input.clone()} minlength="3" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post} oninput={ctx.link().callback(|_| Msg::InputChanged)} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::InputChanged }})} />
                  <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
        <div class="bg-gradient-to-r from-indigo-300 to-indigo-600 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}></div>
        </div>
                      </div>
                      <div class="flex justify-center">
                      <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::SubmitForm)} disabled={!self.can_post} >

                      {"Post"}
                      </button>
                      </div>
                      <div hidden={!self.error_on_input} >
                      <p class="text-red-500">{"Your message is either to short, or to long, ensure it matches the requirements"}</p>
                      </div>
                  </div>
                {self.received_messages.iter().map(|m| html!{<p>{m}</p>}).collect::<Html>()}
            </>
        }
    }
}
