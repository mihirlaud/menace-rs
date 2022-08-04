use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum ScoreRequest {
    ScoreTopicMsg((u32, u32, u32)),
}

pub struct ScoreTopic {
    link: AgentLink<ScoreTopic>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ScoreTopic {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ScoreRequest;
    type Output = (u32, u32, u32);

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            ScoreRequest::ScoreTopicMsg(s) => {
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