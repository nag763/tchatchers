use chrono::Timelike;
use linked_hash_set::LinkedHashSet;
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub messages: LinkedHashSet<WsMessage>,
    pub room: String,
}

pub struct Chat;

impl Component for Chat {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {ctx.props().messages
                .iter().map(|m| {
                match &m.message_type {
                    WsMessageType::Receive => html!{<p>{format!("({}:{:02})[{}]: {}", m.timestamp.clone().hour(), m.timestamp.clone().minute(),m.author.clone().unwrap_or_default(), m.content.clone().unwrap_or_default())}</p>},
                    _ => html!{}
                }
            }).collect::<Html>()}
            </>
        }
    }
}
