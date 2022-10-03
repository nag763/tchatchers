use gloo_timers::callback::{Interval, Timeout};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::WsMessage;
use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

const PROGRESS_REFRESH: u32 = 20;
const TIMEOUT: u32 = 3_000;

pub enum Msg {
    IgnoreEvent,
    SubmitForm,
    ReactivateFields,
    UpdateProgress,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub pass_message_to_ws: Callback<String>,
    pub room: String,
    pub user: PartialUser,
}

#[derive(Default)]
pub struct TypeBar {
    can_post: bool,
    input_ref: NodeRef,
    progress_percentage: u32,
    cooldown: Option<Timeout>,
    progress: Option<Interval>,
}

impl Component for TypeBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            can_post: true,
            ..Self::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if !input.check_validity() {
                        return false;
                    }
                    let msg = WsMessage {
                        room: Some(ctx.props().room.clone()),
                        author: Some(ctx.props().user.clone().into()),
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
        html! {
            <>
                <div />
                <div class="col-span-6 my-6">
                      <input class="shadow appearance-none border dark:border-zinc-800 rounded-xl px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-zinc-900 w-full h-10 invalid:border-red-500 disabled:bg-gray-100 dark:disabled:bg-zinc-700 focus:invalid:border-red-500 bg-gray-200 dark:bg-zinc-700 dark:text-gray-200" type="text" placeholder={placeholder_input} minlength="2" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::IgnoreEvent }})}/>
                      <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
                        <div class="bg-gradient-to-r from-zinc-600 to-zinc-700 dark:from-zinc-200 dark:to-zinc-300 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}>
                        </div>
                    </div>
                </div>
        </>
        }
    }
}
