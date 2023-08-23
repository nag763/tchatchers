// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::rc::Rc;

use tchatchers_core::locale::TranslationMap;
use tchatchers_core::ws_message::WsMessage;
use tchatchers_core::{user::PartialUser, ws_message::WsMessageContent};
use web_sys::HtmlInputElement;
use yew::{html, AttrValue, Callback, Component, Context, Html, NodeRef, Properties};

pub enum Msg {
    SubmitForm,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub pass_message_to_ws: Callback<WsMessage>,
    pub room: AttrValue,
    pub user: PartialUser,
    pub translation: Rc<TranslationMap>,
}

#[derive(Default)]
pub struct TypeBar {
    input_ref: NodeRef,
}

impl Component for TypeBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if !input.check_validity() || input.value().is_empty() {
                        return false;
                    }
                    let msg = WsMessageContent {
                        room: ctx.props().room.to_string(),
                        author: ctx.props().user.clone(),
                        content: input.value(),
                        ..WsMessageContent::default()
                    };
                    ctx.props().pass_message_to_ws.emit(WsMessage::Send(msg));
                    input.set_value("");
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="col-span-6 mb-6">
                <form onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">
                      <input class="shadow appearance-none border dark:border-zinc-800 rounded-xl px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-zinc-900 w-full h-10 invalid:border-red-500 disabled:bg-gray-100 dark:disabled:bg-zinc-700 focus:invalid:border-red-500 bg-gray-200 dark:bg-zinc-700 dark:text-gray-200 dark:carret-indigo-500" type="text" placeholder={ctx.props().translation.as_ref().clone().get_or_default("type_msg_here", "Type a message here")} minlength="2" maxlength="127" ref={self.input_ref.clone()} />
                      <button type="submit" hidden=true></button>
                  </form>
                </div>
        </>
        }
    }
}
