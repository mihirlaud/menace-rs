use yew::prelude::*;
use wasm_logger;
use std::panic;
use console_error_panic_hook;

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
                <div class="left">
                    <Board />
                    <Scoreboard />
                </div>
                <div class="right">
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