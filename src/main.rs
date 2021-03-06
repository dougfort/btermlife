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
    state.live_cells = glider();

    main_loop(context, state)
}

pub fn step(current: &[Point]) -> Vec<Point> {
    let mut neighbor_map: HashMap<Point, CellState> = HashMap::new();

    for point in current {
        for x in [(point.x - 1), point.x, (point.x + 1)].iter() {
            for y in [(point.y - 1), point.y, (point.y + 1)].iter() {
                let neighbor = compute_neighbor(*x, *y);
                let state = &mut neighbor_map.entry(neighbor).or_insert(CellState {
                    live: false,
                    count: 0,
                });
                if neighbor == *point {
                    state.live = true;
                } else {
                    state.count += 1;
                }
            }
        }
    }

    neighbor_map
        .iter()
        .filter(|(_, state)| {
            matches!(
                state,
                CellState {
                    live: true,
                    count: 2,
                } | CellState { count: 3, .. }
            )
        })
        .map(|(cell, _)| *cell)
        .collect()
}

fn compute_neighbor(x: i32, y: i32) -> Point {
    let adjusted_x = match x {
        x if x < 0 => SCREEN_WIDTH - 1,
        x if x > SCREEN_WIDTH - 1 => 0,
        _ => x,
    };
    let adjusted_y = match y {
        y if y < 0 => SCREEN_HEIGHT - 1,
        y if y > SCREEN_HEIGHT - 1 => 0,
        _ => y,
    };
    Point::new(adjusted_x, adjusted_y)
}

fn glider() -> Vec<Point> {
    // glider
    //
    //  *
    //   *
    // ***
    //           (1, ymax-2)
    //                     (2, ymax-1)
    // (0, ymax) (1, ymax) (2, ymax)
    let ymax: i32 = SCREEN_HEIGHT - 1;
    let points = vec![
        Point::new(0, ymax),
        Point::new(1, ymax),
        Point::new(2, ymax),
        Point::new(2, ymax - 1),
        Point::new(1, ymax - 2),
    ];
    points
}
