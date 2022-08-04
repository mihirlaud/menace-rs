use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::collections::{HashMap, HashSet};
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum MBRequest {
    MBTopicMsg(HashMap<String, Vec<usize>>),
}

pub struct MBTopic {
    link: AgentLink<MBTopic>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for MBTopic {
    type Reach = Context<Self>;
    type Message = ();
    type Input = MBRequest;
    type Output = HashMap<String, Vec<usize>>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            MBRequest::MBTopicMsg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
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