use bracket_lib::prelude::*;
use std::collections::HashMap;

struct State {
    live_cells: Vec<Point>,
}

impl State {
    fn new() -> Self {
        State {
            live_cells: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct CellState {
    live: bool,
    count: u32,
}

impl GameState for State {
    fn tick(&mut self, _ctx: &mut BTerm) {
        let mut neighbor_map: HashMap<Point, CellState> = HashMap::new();

    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Life").build()?;
    let mut state = State::new();

    main_loop(context, state)
}
