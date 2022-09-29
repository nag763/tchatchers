use yew::{html, Component, Context, Html, Properties};
use tchatchers_core::ws_message::WsMessage;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub messages: Vec<WsMessage>,
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
                {ctx.props().clone().messages.iter().map(|m| html!{<p>{format!("[{}]: {}", m.author.clone().unwrap_or_default(), m.content.clone().unwrap_or_default())}</p>}).collect::<Html>()}
            </>
        }
    }
}
