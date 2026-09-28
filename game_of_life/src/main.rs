mod grid;
mod screen;

use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::grid::Grid;
use crate::screen::Timings;

const TICK: Duration = Duration::from_millis(100);

fn main() {
    let (width, height) = screen::grid_size();
    let mut grid = Grid::new(width, height);
    grid.randomize();

    screen::setup();

    let mut timings = Timings::default();

    loop {
        let start = Instant::now();
        screen::draw(&grid, &timings);
        timings.draw = start.elapsed();

        let start = Instant::now();
        grid.next_gen();
        timings.next_gen = start.elapsed();

        if esc_pressed() {
            break;
        }
    }

    screen::restore();
}

fn esc_pressed() -> bool {
    if !event::poll(TICK).unwrap() {
        return false;
    }

    match event::read().unwrap() {
        Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => true,
        _ => false,
    }
}
