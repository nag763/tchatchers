// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::components::common::{Form, FormSection};
use crate::router::Route;
use crate::{components::common::FormButton, utils::client_context::ClientContext};
use std::rc::Rc;
use tchatchers_core::{room::RoomNameValidator, validation_error_message::ValidationErrorMessage};
use validator::Validate;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, NodeRef, Properties,
};
use yew_router::scope_ext::RouterScopeExt;

#[function_component(JoinRoomHOC)]
pub fn join_room_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    html! { <JoinRoom  user_context={(*client_context).clone()} /> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    user_context: ClientContext,
}

pub enum Msg {
    SubmitForm,
    VerificationError(String),
}

#[derive(Default)]
pub struct JoinRoom {
    room_name: NodeRef,
    verification_error: Option<AttrValue>,
}

impl Component for JoinRoom {
    type Message = Msg;
    type Properties = Props;

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
                self.verification_error = Some(error.into());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = &ctx.props().user_context.translation;
        html! {
            <Form label="join_a_room_title" {translation} default="Join a room" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} form_error={&self.verification_error} >
                <FormSection label={"room_name"} {translation} default={"Room name"} minlength="1" attr_ref={&self.room_name} required=true />
                <FormButton label={"join_room"} default={"Join"} {translation} />
            </Form>
        }
    }
}
