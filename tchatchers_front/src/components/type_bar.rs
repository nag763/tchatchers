use crate::components::common::FileAttacher;
use crate::router::Route;
use crate::utils::jwt::get_jwt_public_part;
use gloo_timers::callback::{Interval, Timeout};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::WsMessage;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;

const PROGRESS_REFRESH: u32 = 20;
const TIMEOUT: u32 = 5_000;

pub enum Msg {
    FileAttached(Option<js_sys::ArrayBuffer>),
    IgnoreEvent,
    SubmitForm,
    ReactivateFields,
    UpdateProgress,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub pass_message_to_ws: Callback<String>,
    pub jwt: String,
    pub room: String,
}

#[derive(Default)]
pub struct TypeBar {
    file: Option<js_sys::ArrayBuffer>,
    can_post: bool,
    input_ref: NodeRef,
    progress_percentage: u32,
    cooldown: Option<Timeout>,
    progress: Option<Interval>,
    user: PartialUser,
}

impl Component for TypeBar {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let document_cookies = html_document.cookie().unwrap();
        let cookies = &mut document_cookies.split(';');
        let mut user: PartialUser = PartialUser::default();
        for cookie in cookies.by_ref() {
            if let Some(i) = cookie.find('=') {
                let (key, val) = cookie.split_at(i + 1);
                if key == "jwt=" {
                    user = get_jwt_public_part(val.into());
                }
            }
        }
        if user == PartialUser::default() {
            ctx.link().history().unwrap().push(Route::SignIn);
        }
        Self {
            user,
            can_post: true,
            ..Self::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileAttached(file_path) => {
                self.file = file_path;
                true
            }
            Msg::SubmitForm => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if !input.check_validity() {
                        return false;
                    }
                    let msg = WsMessage {
                        jwt: Some(ctx.props().jwt.clone()),
                        room: Some(ctx.props().room.clone()),
                        author: Some(self.user.clone().into()),
                        content: Some(input.value()),
                        ..WsMessage::default()
                    };
                    ctx.props()
                        .pass_message_to_ws
                        .emit(serde_json::to_string(&msg).unwrap());
                    input.set_value("");
                }
                self.cooldown = Some({
                    let link = ctx.link().clone();
                    Timeout::new(TIMEOUT, move || link.send_message(Msg::ReactivateFields))
                });
                self.progress = Some({
                    let link = ctx.link().clone();
                    Interval::new(PROGRESS_REFRESH, move || {
                        link.send_message(Msg::UpdateProgress);
                    })
                });

                self.file = None;
                self.can_post = false;
                self.progress_percentage = 0;
                true
            }
            Msg::UpdateProgress => {
                self.progress_percentage += PROGRESS_REFRESH;
                true
            }
            Msg::ReactivateFields => {
                self.can_post = true;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let placeholder_input = match self.can_post {
            true => "Type a message here",
            false => "Wait few seconds before typing...",
        };
        let link = ctx.link().clone();
        let on_file_attached = Callback::from(move |file_path: Option<js_sys::ArrayBuffer>| {
            link.send_message(Msg::FileAttached(file_path));
        });
        html! {
            <>
                <div class="grid justify-items-center content-center">
                    <FileAttacher {on_file_attached} disabled={!self.can_post} />
                </div>
                <div class="col-span-4">
                      <input class="shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-indigo-500 w-full h-10 invalid:border-red-500 disabled:bg-gray-100 focus:invalid:border-red-500" id="username" type="text" placeholder={placeholder_input} minlength="2" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::IgnoreEvent }})}/>
                      <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
                        <div class="bg-gradient-to-r from-indigo-300 to-indigo-600 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}>
                        </div>
                    </div>
                </div>
            <div class="flex justify-center">
              <button class="bg-indigo-500 enabled:hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::SubmitForm)} disabled={!self.can_post} >

                  {"Post"}
              </button>
          </div>
        </>
        }
    }
}
