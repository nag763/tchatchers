// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
use yew::Properties;
use yew_agent::worker::{HandlerId, Worker, WorkerScope};

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ProfileRMenuProps {
    pub user_id: i32,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct MessageRMenuProps {
    pub message_id: Uuid,
    pub is_self: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RMenuKind {
    MessageRMenu(MessageRMenuProps),
    ProfileRMenu(ProfileRMenuProps),
}

pub struct RMenuBus {
    subscribers: HashSet<HandlerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RMenusBusEvents {
    OpenRMenu(i32, i32, RMenuKind),
    CloseRMenu,
}

impl Worker for RMenuBus {
    type Message = ();
    type Input = RMenusBusEvents;
    type Output = RMenusBusEvents;

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
