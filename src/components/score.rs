use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::topics::score_topic::ScoreTopic;

pub struct Scoreboard {
    wins: u32,
    losses: u32,
    draws: u32,
    _pub: Box<dyn Bridge<ScoreTopic>>
}

#[derive(Properties, PartialEq)]
pub struct ScoreboardProps {
}

pub enum ScoreboardMsg {
    Update((u32, u32, u32))
}

impl Component for Scoreboard {
    type Message = ScoreboardMsg;
    type Properties = ScoreboardProps;

    fn create(ctx: &Context<Self>) -> Self {

        Self {
            wins: 0,
            losses: 0,
            draws: 0,
            _pub: ScoreTopic::bridge(ctx.link().callback(ScoreboardMsg::Update))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ScoreboardMsg::Update((w, l, d)) => {
                self.wins += w;
                self.losses += l;
                self.draws += d;

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="scoreboard">
                <p>{format!("Human : {}-{}-{} : Computer", self.wins, self.draws, self.losses)}</p>
            </div>
        }
    }
}