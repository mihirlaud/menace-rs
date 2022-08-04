use yew::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use yew_agent::{Dispatched, Dispatcher};
use crate::topics::score_topic::{ScoreTopic, ScoreRequest};
use crate::topics::mb_topic::{MBTopic, MBRequest};

#[derive(Clone, Copy)]
struct BoardCell {
    token: char,
    disabled: bool,
}

impl BoardCell {
    fn default() -> Self {
        Self {
            token: '-',
            disabled: false,
        }
    }
}

pub struct Board {
    token: char,
    cells: Vec<Vec<BoardCell>>,
    matchboxes: HashMap<String, Vec<usize>>,
    score_topic: Dispatcher<ScoreTopic>,
    mb_topic: Dispatcher<MBTopic>,
    last_menace_move: (String, usize),
    reset: bool,
    self_play: bool,
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
}

pub enum BoardMsg {
    Click(usize, usize),
    UpdateScore((u32, u32, u32)),
    UpdateMatchboxes(HashMap<String, Vec<usize>>),
    EndGame,
    Reset,
}

fn rotate_string(board: String) -> String {
    /*
    
    012    630
    345 -> 741
    678    852

    012345678 -> 630741852
    */

    vec![
        board.chars().nth(6).unwrap(),
        board.chars().nth(3).unwrap(),
        board.chars().nth(0).unwrap(),
        board.chars().nth(7).unwrap(),
        board.chars().nth(4).unwrap(),
        board.chars().nth(1).unwrap(),
        board.chars().nth(8).unwrap(),
        board.chars().nth(5).unwrap(),
        board.chars().nth(2).unwrap(),
    ].iter().cloned().collect::<String>()
}

fn reflect_string(board: String) -> String {
    /*
    012    678
    345 -> 345
    678    012

    012345678 -> 678345012
    */

    vec![
        board.chars().nth(6).unwrap(),
        board.chars().nth(7).unwrap(),
        board.chars().nth(8).unwrap(),
        board.chars().nth(3).unwrap(),
        board.chars().nth(4).unwrap(),
        board.chars().nth(5).unwrap(),
        board.chars().nth(0).unwrap(),
        board.chars().nth(1).unwrap(),
        board.chars().nth(2).unwrap(),
    ].iter().cloned().collect::<String>()
}

fn get_permutations(board: &String) -> Vec<(String, i32, i32)> {
    let mut perms = vec![];

    for n in 0 .. 8 {
        let mut perm = board.clone();
        if n % 2 == 1 {
            perm = reflect_string(perm);
        }
        for _ in 0 .. n / 2 {
            perm = rotate_string(perm);
        }
        perms.push((perm, n % 2, n / 2));
    }

    perms
}

impl Board {
    fn as_string(&self) -> String {
        self.cells.iter().map(|row| {
            row.iter().map(|cell| cell.token).collect::<String>()
        }).collect::<String>()
    }

    fn take_turn(&mut self, ctx: &Context<Self>) {
        let mut current_board = self.as_string();
        let mut refl = false;
        let mut rot = 0;
        let current_perms = get_permutations(&current_board);
        for (perm, perm_refl, perm_rot) in current_perms {
            if self.matchboxes.contains_key(&perm) {
                current_board = perm;
                refl = if perm_refl == 1 { true } else { false };
                rot = perm_rot;
            }
        }
        
        match self.matchboxes.get(&current_board) {
            Some(beads) => {
                let idx = thread_rng().gen_range(0 .. beads.len());
                let board_idx = beads[idx];
                let mut i = board_idx / 3;
                let mut j = board_idx % 3;

                for _ in 0 .. 4 - rot {
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

                if refl {
                    i = 2 - i;
                }

                ctx.link().send_message(BoardMsg::Click(i, j));

                self.last_menace_move = (current_board, board_idx);
            }
            None => {
                let mut beads = vec![];

                for (idx, c) in current_board.chars().enumerate() {
                    if c == '-' {
                        beads.push(idx);
                    }
                }

                let idx = thread_rng().gen_range(0..beads.len());
                let board_idx = beads[idx];
                let i = board_idx / 3;
                let j = board_idx % 3;
                
                ctx.link().send_message(BoardMsg::Click(i, j));
                
                self.last_menace_move = (current_board.clone(), board_idx);

                self.matchboxes.insert(current_board, beads);
                ctx.link().send_message(BoardMsg::UpdateMatchboxes(self.matchboxes.clone()))
            }
        }

    }

    fn check_win(&mut self, ctx: &Context<Self>) -> bool {
        let board: Vec<char> = self.as_string().chars().collect();

        let mut has_win = false;
        let mut winner = '-';

        let win_conditions = vec![
            (0, 1, 2), (3, 4, 5), (6, 7, 8),
            (0, 3, 6), (1, 4, 7), (2, 5, 8),
            (0, 4, 8), (2, 4, 6),
        ];

        for (i, j, k) in win_conditions {
            if board[i] != '-' && (board[i] == board[j] && board[i] == board[k]) {        
                has_win = true;
                winner = board[i];
                break;
            }
        }

        if !has_win && !board.contains(&'-') {
            winner = 'D';
            has_win = true;
        }

        match winner {
            'X' => {
                ctx.link().send_message(BoardMsg::UpdateScore((0, 1, 0)));
                let beads = self.matchboxes.get(&self.last_menace_move.0).unwrap();
                let mut new_beads = beads.clone();
                new_beads.append(&mut vec![self.last_menace_move.1; 3]);

                self.matchboxes.insert(self.last_menace_move.0.clone(), new_beads);
                ctx.link().send_message(BoardMsg::UpdateMatchboxes(self.matchboxes.clone()));
            }
            'O' => {
                ctx.link().send_message(BoardMsg::UpdateScore((1, 0, 0)));
                let beads = self.matchboxes.get(&self.last_menace_move.0).unwrap();
                let mut new_beads = beads.clone();
                new_beads.retain(|&x| x != self.last_menace_move.1);

                self.matchboxes.insert(self.last_menace_move.0.clone(), new_beads);
                ctx.link().send_message(BoardMsg::UpdateMatchboxes(self.matchboxes.clone()));
            }
            'D' => {
                ctx.link().send_message(BoardMsg::UpdateScore((0, 0, 1)));
                let beads = self.matchboxes.get(&self.last_menace_move.0).unwrap();
                let mut new_beads = beads.clone();
                new_beads.append(&mut vec![self.last_menace_move.1]);

                self.matchboxes.insert(self.last_menace_move.0.clone(), new_beads);
                log::info!("{:?}", self.matchboxes);
                ctx.link().send_message(BoardMsg::UpdateMatchboxes(self.matchboxes.clone()));
            }
            _ => ()
        }

        has_win
    }
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut board = Board {
            token: 'X',
            cells: vec![
                vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
                vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
                vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
            ],
            matchboxes: HashMap::new(),
            score_topic: ScoreTopic::dispatcher(),
            mb_topic: MBTopic::dispatcher(),
            last_menace_move: (String::new(), 0),
            reset: false,
            self_play: false,
        };

        board.take_turn(ctx);

        board
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Click(i, j) => {
                self.cells[i][j].token = self.token;
                self.cells[i][j].disabled = true;
                if self.token == 'X' {
                    self.token = 'O';
                } else {
                    self.token = 'X';
                }

                if self.check_win(ctx) {
                    ctx.link().send_message(BoardMsg::EndGame);
                    return true;
                }

                if self.token == 'X' {
                    self.take_turn(ctx);
                } else if self.token == 'O' && self.self_play {
                    self.take_turn(ctx);
                }

                true
            },
            BoardMsg::UpdateScore(u) => {
                self.score_topic.send(ScoreRequest::ScoreTopicMsg(u.to_owned()));
                false
            },
            BoardMsg::UpdateMatchboxes(m) => {
                self.mb_topic.send(MBRequest::MBTopicMsg(m.to_owned()));
                false
            },
            BoardMsg::EndGame => {
                for i in 0 .. self.cells.len() {
                    for j in 0 .. self.cells[i].len() {
                        self.cells[i][j].disabled = true;
                    }
                }
                self.reset = true;

                true
            },
            BoardMsg::Reset => {
                self.token = 'X';
                self.cells = vec![
                    vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
                    vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
                    vec![BoardCell::default(), BoardCell::default(), BoardCell::default()],
                ];
                self.reset = false;

                self.take_turn(ctx);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <table class="board" >
                {self.cells.iter().enumerate().map(|(i, row)| {
                    html! {
                        <tr>
                            {row.iter().enumerate().map(|(j, cell)| {
                                html! {
                                    <button class="board-cell" disabled={cell.disabled} onclick={link.callback(move |_| BoardMsg::Click(i, j))} >{cell.token}</button>
                                }
                            }).collect::<Html>()}
                        </tr>
                    }
                }).collect::<Html>()}    
            </table>
            <button disabled={!self.reset} onclick={ctx.link().callback(|_| BoardMsg::Reset)}>
                { "Reset" }
            </button>
            </div>
        }
    }
}