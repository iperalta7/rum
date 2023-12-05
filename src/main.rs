use std::env;
use rum::{load, rumdis};
use rum::state::UniversalMachine;

fn main() {
    let input = env::args().nth(1);
    let instructions = load::load(input.as_deref());
    let mut state = UniversalMachine::new();
    rumdis::run(&mut state, instructions.clone())
}
