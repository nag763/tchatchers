// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::FormButton;
use crate::router::Route;
use tchatchers_core::{room::RoomNameValidator, validation_error_message::ValidationErrorMessage};
use validator::Validate;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};
use yew_router::scope_ext::RouterScopeExt;

pub enum Msg {
    SubmitForm,
    VerificationError(String),
}

#[derive(Default)]
pub struct JoinRoom {
    room_name: NodeRef,
    verification_error: Option<String>,
}

impl Component for JoinRoom {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                self.verification_error = None;
                if let Some(room_name) = self.room_name.cast::<HtmlInputElement>() {
                    if room_name.check_validity() {
                        let room_name = room_name.value();
                        if let Err(e) = RoomNameValidator::from(room_name.clone()).validate() {
                            ctx.link().send_message(Msg::VerificationError(
                                ValidationErrorMessage::from(e).to_string(),
                            ));
                        } else {
                            ctx.link().navigator().unwrap().push(&Route::Room {
                                room: room_name.to_lowercase(),
                            });
                        }
                    }
                }
            }
            Msg::VerificationError(error) => {
                self.verification_error = Some(error);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="flex items-center justify-center h-full dark:bg-zinc-800">
                <form class="w-full max-w-sm border-2 dark:border-zinc-700 px-6 py-6 lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">

                <h2 class="text-xl mb-10 text-center text-gray-500 dark:text-gray-200 font-bold">{"Join a room"}</h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Room name"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="1" ref={&self.room_name} />
                    </div>
                  </div>
                    <small class="flex mt-4 mb-2 items-center text-red-500" hidden={self.verification_error.is_none()}>
                        {self.verification_error.as_ref().unwrap_or(&String::new())}
                    </small>
                  <FormButton label="Join room" />
                </form>
                </div>
            </>
        }
    }
}
