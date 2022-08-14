use yew::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use crate::topics::mb_topic::MBTopic;
use yew_agent::{Bridge, Bridged};

pub struct Matchboxes {
    matchboxes: HashMap<String, Vec<usize>>,
    last_move: String,
    last_perm: String,
    refl: bool,
    rot: i32,
    _pub: Box<dyn Bridge<MBTopic>>
}

#[derive(Properties, PartialEq)]
pub struct MBProps {
}

pub enum MBMsg {
    Update((HashMap<String, Vec<usize>>, String))
}

impl Component for Matchboxes {
    type Message = MBMsg;
    type Properties = MBProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            matchboxes: HashMap::new(),
            last_move: String::new(),
            last_perm: String::new(),
            refl: false,
            rot: 0,
            _pub: MBTopic::bridge(ctx.link().callback(MBMsg::Update))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MBMsg::Update((map, last_move)) => {
                self.matchboxes = map;
                if last_move != self.last_perm {
                    self.last_move = last_move;

                    self.refl = false;
                    self.rot = 0;
                    let current_perms = crate::components::board::get_permutations(&self.last_move);
                    for (perm, perm_refl, perm_rot) in current_perms {
                        if self.matchboxes.contains_key(&perm) {
                            self.last_perm = perm;
                            self.refl = if perm_refl == 1 { true } else { false };
                            self.rot = perm_rot;
                        }
                    }
                }

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::info!("{}", self.last_move);
        
        if let Some(beads) = self.matchboxes.get(&self.last_perm) {
            let mut freq: HashMap<usize, i32> = HashMap::new();
            for b in beads {
                match freq.get(b) {
                    Some(f) => {
                        freq.insert(*b, f + 1);
                    }
                    None => {
                        freq.insert(*b, 1);
                    }
                }
            }

            let mut arr: [[i32; 3]; 3] = [[0; 3]; 3];
            for key in freq.keys() {
                let mut i = key / 3;
                let mut j = key % 3;

                for _ in 0 .. 4 - self.rot {
                    (i, j) = match (i, j) {
                        (0, 0) => (0, 2),
                        (0, 1) => (1, 2),
                        (0, 2) => (2, 2),
                        (1, 0) => (0, 1),
                        (1, 2) => (2, 1),
                        (2, 0) => (0, 0),
                        (2, 1) => (1, 0),
                        (2, 2) => (2, 0),
                        _ => (i, j)
                    };
                }

                if self.refl {
                    i = 2 - i;
                }

                arr[i][j] = *freq.get(key).unwrap();
            }

            html! {
                <div class="menace-vis">
                    <p style="text-align: center;">{"PREVIOUS MOVE"}</p>
                    <table class="menace-vis-table">
                    {
                        for arr.iter().enumerate().map(|(i, row)| {
                            html! {
                                <tr>
                                {
                                    for row.iter().enumerate().map(|(j, c)| {
                                        html! { 
                                            <td class="menace-vis-cell">
                                            {
                                                if *c != 0 { 
                                                    format!("{:.1}%", *c as f32 / beads.len() as f32 * 100.0) 
                                                } else {
                                                    self.last_move.chars().nth(3 * i + j).unwrap().to_string()
                                                 }
                                            }
                                            </td>
                                        }
                                    })
                                }
                                </tr>
                            }
                        })
                    }
                    </table>
                </div>
            }
        } else {
            html! {
                <div class="matchboxes">
                </div>
            }
        }
    }
}