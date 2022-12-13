// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use super::message::{WsBusMessage, WsBusMessageType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(String),
    EventBusErr(String),
    EventBusKeepAlive,
    EventBusNotConnected,
    EventBusClosed,
}

pub struct EventBus {
    link: WorkerLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for EventBus {
    type Message = ();
    type Input = Request;
    type Output = String;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg(s) => {
                let msg = WsBusMessage {
                    message_type: WsBusMessageType::Reply,
                    content: s,
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone())
                }
            }
            Request::EventBusKeepAlive => {
                let msg = WsBusMessage {
                    message_type: WsBusMessageType::Pong,
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
                let msg = WsBusMessage {
                    message_type: WsBusMessageType::NotConnected,
                    content: String::from("Error on connection"),
                };
                let serialized_message = serde_json::to_string(&msg).unwrap();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, serialized_message.clone());
                }
            }
            Request::EventBusClosed => {
                let msg = WsBusMessage {
                    message_type: WsBusMessageType::Closed,
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

    fn name_of_resource() -> &'static str {
        "chat_worker.js"
    }
}
