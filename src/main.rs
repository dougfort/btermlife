use bracket_lib::prelude::*;
use std::collections::HashMap;
use std::time;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

struct State {
    tick: time::Duration,
    elapsed: time::Duration,
    live_cells: Vec<Point>,
}

impl State {
    fn new() -> Self {
        State {
            tick: time::Duration::new(1, 0),
            elapsed: Default::default(),
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
    fn tick(&mut self, ctx: &mut BTerm) {
        let frame_duration = time::Duration::from_millis(ctx.frame_time_ms as u64);
        self.elapsed += frame_duration;
        if self.elapsed > self.tick {
            ctx.cls();
            self.live_cells = step(&self.live_cells);
            for point in &self.live_cells {
                ctx.set(
                    point.x,
                    point.y,
                    RGB::named(YELLOW),
                    RGB::named(BLACK),
                    to_cp437('*'),
                );
            }
            self.elapsed = Default::default();
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Life").build()?;
    let mut state = State::new();
    state.live_cells.push(Point::new(5, 4));
    state.live_cells.push(Point::new(5, 5));
    state.live_cells.push(Point::new(5, 6));

    main_loop(context, state)
}

pub fn step(current: &[Point]) -> Vec<Point> {
    let mut neighbor_map: HashMap<Point, CellState> = HashMap::new();

    for point in current {
        for x in [(point.x - 1), point.x, (point.x + 1)].iter() {
            for y in [(point.y - 1), point.y, (point.y + 1)].iter() {
                let neighbor_cell = Point { x: *x, y: *y };
                let state = &mut neighbor_map.entry(neighbor_cell).or_insert(CellState {
                    live: false,
                    count: 0,
                });
                if neighbor_cell == *point {
                    state.live = true;
                } else {
                    state.count += 1;
                }
            }
        }
    }

    neighbor_map
        .iter()
        .filter(|(_, state)| match state {
            CellState {
                live: true,
                count: 2,
            } => true,
            CellState { count: 3, .. } => true,
            _ => false,
        })
        .map(|(cell, _)| *cell)
        .collect()
}
