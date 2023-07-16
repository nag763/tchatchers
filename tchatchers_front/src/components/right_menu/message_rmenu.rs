use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tchatchers_core::{profile::Profile, locale::Translation, ws_message::WsMessage};
use uuid::Uuid;
use yew::{function_component, html, use_context, Html, Properties};
use yew_agent::Dispatched;

use crate::{
    components::{common::I18N, toast::Alert},
    services::{chat_bus::ChatBus, toast_bus::ToastBus},
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

    let translation: Rc<Translation> = client_context
        .user_context
        .as_ref()
        .unwrap()
        .translation
        .clone();

    let delete_message_li = {
        let bearer = bearer.clone();
        let translation = translation.clone();
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

    let report_message_li = {
        let report_message_id = {
            let props = props.clone();
            move |_| {
                let mut req = Requester::post(&format!("/api/message/{}/report", props.message_id));
                req.bearer(bearer.clone());
                wasm_bindgen_futures::spawn_local(async move {
                    let res = req.send().await;
                    if res.ok() {
                        ToastBus::dispatcher().send(Alert {
                            is_success: true,
                            content: "This message has been reported with success".into(),
                        });
                    } else {
                        ToastBus::dispatcher().send(Alert {
                            is_success: false,
                            content: res.text().await.unwrap_or_else(|_| {
                                "A problem happened while reporting this message".into()
                            }),
                        });
                    }
                })
            }
        };
        html! {
        <li class="hover:text-gray-300" onclick={report_message_id}>
            <I18N label={"report_message"} default={"Report message"} {translation}/>
        </li>}
    };

    match client_context.user_context.as_ref().unwrap().user.profile {
        Profile::Moderator | Profile::Admin => html! {
            <ul>
                {delete_message_li}
            </ul>
        },
        Profile::User => html! {
            <ul>
            if props.is_self {
                {delete_message_li}
            } else {
                {report_message_li}
            }
            </ul>

        },
    }
}
