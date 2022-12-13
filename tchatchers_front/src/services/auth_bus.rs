// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::collections::HashSet;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

pub struct EventBus {
    link: WorkerLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for EventBus {
    type Reach = Public<Self>;
    type Message = ();
    type Input = bool;
    type Output = bool;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg);
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn name_of_resource() -> &'static str {
        "auth_worker.js"
    }
}
