// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::worker::{HandlerId, Worker, WorkerScope};

#[derive(Clone, Serialize, Deserialize)]
pub struct Alert {
    pub is_success: bool,
    pub label: String,
    pub default: String,
}

pub struct ToastBus {
    subscribers: HashSet<HandlerId>,
}

impl Worker for ToastBus {
    type Message = ();
    type Input = Alert;
    type Output = Alert;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, _id: HandlerId) {
        scope.respond(_id, msg.clone());
        for sub in self.subscribers.iter() {
            scope.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, _id: HandlerId) {
        // self.subscribers.remove(&id);
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}
}
