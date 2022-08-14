# MENACE in Rust

## About
The Machine Educable Noughts and Crosses Engine, or **MENACE**, was a matchbox computer made by British computer scientist **Donald Michie** in 1961. Its purpose was to demonstrate the means by which computers could learn to play simple games at and beyond human skill levels through **reinforcement learning**. Learn more about MENACE [here](https://en.wikipedia.org/wiki/Matchbox_Educable_Noughts_and_Crosses_Engine).

This is an implementation of MENACE in the Rust programming language. You can play against MENACE-rs [here](https://menace-rs.netlify.app/). While the desktop version is mostly polished, the mobile version still needs work.

## Installation

To run the project on your locally, you will need to install Rust and Cargo, following [these instructions](https://www.rust-lang.org/tools/install).

Then, add the WebAssembly target with:
```sh
rustup target add wasm32-unknown-unknown
```

Install Trunk for building and serving the project with:
```sh
cargo install trunk
```

Finally, clone the repo, and run the serve bash script.
```sh
git clone https://github.com/mihirlaud/menace-rs.git
cd menace-rs
./serve.sh
```

Trunk will automatically build the project, serve it on localhost:4000, and open in the browser.