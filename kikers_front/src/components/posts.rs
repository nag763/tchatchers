use crate::services::chat::WebsocketService;
use crate::services::event_bus::EventBus;
use gloo_timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};
use yew_agent::{Bridge, Bridged};

#[derive(Clone, PartialEq, Properties)]
pub struct PostProps {
    login: String,
}

struct Post {}

impl Component for Post {
    type Message = ();
    type Properties = PostProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { <div>
        <img />
            <p>{&ctx.props().login}</p>
        </div> }
    }
}

pub struct Posts {
    received_messages: Vec<String>,
    _ws: WebsocketService,
    _sender: Interval,
    _producer: Box<dyn Bridge<EventBus>>,
}

pub enum Msg {
    HandleMsg(String),
}

impl Component for Posts {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ws = WebsocketService::new();
        Self {
            received_messages: vec![],
            _sender: {
                let ws = ws.clone();
                Interval::new(10_000, move || {
                    ws.tx.clone().try_send("Hello there!".to_string()).unwrap();
                })
            },
            _ws: ws,

            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                self.received_messages.push(s);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="row-span-10">
            {self.received_messages.iter().map(|m| html!{<p>{m}</p>}).collect::<Html>()}
                <Post login="" />
            </div>
        }
    }
}
