// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

#[derive(Clone, Serialize, Deserialize)]
pub struct Alert {
    pub is_success: bool,
    pub label: String,
    pub default: String,
}

pub struct ToastBus {
    link: WorkerLink<ToastBus>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for ToastBus {
    type Reach = Public<Self>;
    type Message = ();
    type Input = Alert;
    type Output = Alert;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn name_of_resource() -> &'static str {
        "toast_service.js"
    }
}
