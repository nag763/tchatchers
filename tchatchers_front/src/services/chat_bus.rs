// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::collections::HashSet;
use tchatchers_core::ws_message::WsMessage;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

pub struct ChatBus {
    link: WorkerLink<ChatBus>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for ChatBus {
    type Message = ();
    type Input = WsMessage;
    type Output = WsMessage;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone())
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
