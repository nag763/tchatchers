use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tchatchers_core::{profile::Profile, ws_message::WsMessage};
use uuid::Uuid;
use yew::{function_component, html, use_context, Html, Properties};
use yew_agent::Dispatched;

use crate::{
    components::common::I18N,
    services::chat_bus::ChatBus,
    utils::{client_context::ClientContext, requester::Requester},
};

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct MessageRMenuProps {
    pub message_id: Uuid,
    pub is_self: bool,
}

#[function_component(MessageRMenu)]
pub fn message_rmenu(props: &MessageRMenuProps) -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let bearer = client_context.bearer.clone();

    let translation = client_context
        .user_context
        .as_ref()
        .unwrap()
        .translation
        .clone();

    let delete_message_li = {
        let delete_message_id = {
            let props = props.clone();
            move |_| {
                let mut req = Requester::delete(&format!("/api/message/{}", props.message_id));
                req.bearer(bearer.clone());
                wasm_bindgen_futures::spawn_local(async move {
                    let res = req.send().await;
                    if res.ok() {
                        ChatBus::dispatcher().send(WsMessage::Delete(props.message_id));
                    }
                })
            }
        };
        html! {
        <li class="hover:text-gray-300" onclick={delete_message_id}>
            <I18N label={"delete_message"} default={"Delete message"} {translation}/>
        </li>}
    };

    match client_context.user_context.as_ref().unwrap().user.profile {
        Profile::Moderator | Profile::Admin => html! {
            <ul>
                {delete_message_li}
            </ul>
        },
        Profile::User => html! {
            if props.is_self {
                <ul>
                    {delete_message_li}
                </ul>
            }
        },
    }
}
