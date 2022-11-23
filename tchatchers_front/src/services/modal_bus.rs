// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::components::modal::ModalContent;
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

pub struct ModalBus {
    link: AgentLink<ModalBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ModalBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ModalContent;
    type Output = ModalContent;

    fn create(link: AgentLink<Self>) -> Self {
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
}