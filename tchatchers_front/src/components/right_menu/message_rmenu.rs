use std::rc::Rc;

use rmenu_service::MessageRMenuProps;
use tchatchers_core::{api_response::ApiResponse, profile::Profile, ws_message::WsMessage};
use yew::{function_component, html, use_context, Html};
use yew_agent::Dispatched;

use crate::{
    components::common::I18N,
    utils::{client_context::ClientContext, requester::Requester},
};
use chat_service::bus::ChatBus;
use toast_service::{Alert, ToastBus};

#[function_component(MessageRMenu)]
pub fn message_rmenu(props: &MessageRMenuProps) -> Html {
    let client_context = use_context::<Rc<ClientContext>>().unwrap();

    let bearer = client_context.bearer.clone();

    let translation = &client_context.translation;

    let delete_message_li = {
        let bearer = bearer.clone();
        let delete_message_id = {
            let props = props.clone();
            move |_| {
                let mut req = Requester::delete(&format!("/api/message/{}", props.message_id));
                req.bearer(bearer.clone());
                wasm_bindgen_futures::spawn_local(async move {
                    let res = req.send().await;
                    let api_resp: ApiResponse =
                        bincode::deserialize(&res.binary().await.unwrap()).unwrap();
                    let label = api_resp.label;
                    let default: String = api_resp.text.unwrap_or("Unknown response".into());
                    let is_success = res.ok();
                    ToastBus::dispatcher().send(Alert {
                        is_success,
                        label,
                        default,
                    });
                    if is_success {
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
                    let api_resp: ApiResponse =
                        bincode::deserialize(&res.binary().await.unwrap()).unwrap();
                    let label = api_resp.label;
                    let default: String = api_resp.text.unwrap_or("Unknown response".into());
                    let is_success = res.ok();
                    ToastBus::dispatcher().send(Alert {
                        is_success,
                        label,
                        default,
                    });
                })
            }
        };
        html! {
        <li class="hover:text-gray-300" onclick={report_message_id}>
            <I18N label={"report_message"} default={"Report message"} {translation}/>
        </li>}
    };

    match client_context.user.as_ref().unwrap().profile {
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
