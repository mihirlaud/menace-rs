use yew::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use crate::topics::mb_topic::MBTopic;
use yew_agent::{Bridge, Bridged};

pub struct Matchboxes {
    matchboxes: Vec<String>,
    _pub: Box<dyn Bridge<MBTopic>>
}

#[derive(Properties, PartialEq)]
pub struct MBProps {
}

pub enum MBMsg {
    Update(HashMap<String, Vec<usize>>)
}

impl Component for Matchboxes {
    type Message = MBMsg;
    type Properties = MBProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            matchboxes: vec![],
            _pub: MBTopic::bridge(ctx.link().callback(MBMsg::Update))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MBMsg::Update(map) => {
                self.matchboxes = vec![];
                for key in map.keys() {
                    let mut freq: HashMap<usize, u32> = HashMap::new();
                    let beads = map.get(key).unwrap();
                    for bead in beads {
                        match freq.get(bead) {
                            Some(f) => {
                                freq.insert(*bead, f + 1);
                            }
                            None => {
                                freq.insert(*bead, 1);
                            }
                        }
                    }
                    let mut prob_vec: Vec<String> = freq.keys().map(|k| format!("{},{:.2};", k, *freq.get(k).unwrap() as f32 / beads.len() as f32)).collect();
                    prob_vec.sort();
                    self.matchboxes.push(format!("{}:{}", key, prob_vec.iter().cloned().collect::<String>()));
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        html! {
            {
                self.matchboxes.iter().map(|s| {
                    let split_1: Vec<&str> = s.split(':').collect();
                    let board = split_1[0];

                    let split_2: Vec<&str> = split_1[1].split(';').collect();
                    let split_3 = split_2.iter().map(|s| {
                        s.split(',').collect::<Vec<&str>>()
                    }).collect::<Vec<Vec<&str>>>();

                    let mut square: Vec<Vec<String>> = vec![
                        vec![board.get(0..1).unwrap().to_string(), board.get(1..2).unwrap().to_string(), board.get(2..3).unwrap().to_string()],
                        vec![board.get(3..4).unwrap().to_string(), board.get(4..5).unwrap().to_string(), board.get(5..6).unwrap().to_string()],
                        vec![board.get(6..7).unwrap().to_string(), board.get(7..8).unwrap().to_string(), board.get(8..9).unwrap().to_string()],
                    ];

                    for pair in split_3 {
                        if pair.len() > 1 {
                            let idx = usize::from_str_radix(pair[0], 10).unwrap();
                            let i = idx / 3;
                            let j = idx % 3;
                            square[i][j] = pair[1].to_string();
                        }
                    }
                    
                    html! {
                        <div class="matchbox-wrapper">
                            <table class="matchbox">
                                {square.iter().map(|row| {
                                    html! {
                                        <tr>
                                            {row.iter().map(|cell| {
                                                html!{ <td class="matchbox-cell">{cell}</td> }
                                            }).collect::<Html>()}
                                        </tr>
                                    }
                                }).collect::<Html>()}
                            </table>
                        </div>
                    }
                }).collect::<Html>()
            }
        }
    }
}