#![recursion_limit = "2048"]
use yew::prelude::*;
use std::panic;

mod components;
mod topics;

use components::board::Board;
use components::matchboxes::Matchboxes;
use components::score::Scoreboard;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div>
                <div class="column">
                    <h1>{"MENACE-rs"}</h1>
                    <p>{"The Machine Educable Noughts and Crosses Engine, or MENACE, was a matchbox computer made by British computer scientist Donald Michie in 1961. Its purpose was to demonstrate the means by which computers could learn to play simple games at and beyond human skill levels through reinforcement learning."}</p>
                    <p>{"This website is an implementation of MENACE in the Rust programming language. The board of buttons allows you to play games as O against the computer as X. The 3x3 square on the right shows the probability that MENACE chose a given square on the previous move."}</p>
                    <p>{"You can learn more about MENACE and how it works "}<a href="https://en.wikipedia.org/wiki/Matchbox_Educable_Noughts_and_Crosses_Engine" target="_blank">{"here."}</a></p>
                    <p>{"Created by "}<a href="https://mihirlaud.github.io/" target="_blank">{"Mihir Laud"}</a>{". Page last updated 2022-08-14."}</p>
                </div>
                <div class="column">
                    <Board />
                    <Scoreboard />
                </div>
                <div class="column">
                    <Matchboxes />
                </div>
            </div>
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}