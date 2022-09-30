use chrono::{Datelike, Timelike};
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub messages: Vec<WsMessage>,
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
                    WsMessageType::Receive =>
            {
            if let Some(author) = &m.author {
                let pfp = match &author.pfp {
                    Some(v) => v.clone(),
                    None => "/assets/no_pfp.webp".into(),
                };
                html!{
            <div class="flex w-full mt-2 space-x-3 max-w-xs px-3">
                <div class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300" title={author.name.clone()}>
                <img class="h-10 w-10 rounded-full" src={pfp.clone()} alt="No img"/>
            </div>
                <div>
                    <div class="bg-gray-300 mb-2 p-3 rounded-r-lg rounded-bl-lg" title={format!("on {:02}/{:02}/{} at {}:{:02}", m.timestamp.clone().day(), m.timestamp.clone().month(), m.timestamp.clone().year(), m.timestamp.clone().hour(), m.timestamp.clone().minute())}>
                        <p class="text-sm">{m.content.clone().unwrap_or_default()}</p>
                    </div>
                </div>
            </div>
            }
            } else {
                html!{<></>}
            }
            }
                    _ => html!{}
                }
            }).collect::<Html>()}
            </>
        }
    }
}
