use std::env::{self};

mod config;
mod logic;

fn main() {
    let args: Vec<String> = env::args().collect();

    logic::run(args);
}
