use super::message::{WsMessage, WsMessageType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(String),
    EventBusErr(String),
    EventBusKeepAlive,
    EventBusNotConnected,
    EventBusClosed,
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg(s) => {
                let msg = WsMessage {
                    message_type: WsMessageType::Reply,
                    content: s.clone(),
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone())
                }
            }
            Request::EventBusKeepAlive => {
                let msg = WsMessage {
                    message_type: WsMessageType::Pong,
                    content: String::from("Pong"),
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone())
                }
            }
            Request::EventBusErr(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone())
                }
            }
            Request::EventBusNotConnected => {
                let msg = WsMessage {
                    message_type: WsMessageType::NotConnected,
                    content: String::from("Error on connection"),
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone());
                }
            }
            Request::EventBusClosed => {
                let msg = WsMessage {
                    message_type: WsMessageType::Closed,
                    content: String::from("Connection closed"),
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
