// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
use yew::Properties;
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

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
    link: WorkerLink<RMenuBus>,
    subscribers: HashSet<HandlerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RMenusBusEvents {
    OpenRMenu(i32, i32, RMenuKind),
    CloseRMenu,
}

impl Worker for RMenuBus {
    type Reach = Public<Self>;
    type Message = ();
    type Input = RMenusBusEvents;
    type Output = RMenusBusEvents;

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
        "rmenu_service.js"
    }

}
