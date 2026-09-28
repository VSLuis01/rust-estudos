mod grid;
mod screen;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::grid::Grid;

const TICK: Duration = Duration::from_millis(100);

fn main() {
    let (width, height) = screen::grid_size();
    let mut grid = Grid::new(width, height);
    grid.randomize();

    screen::setup();

    loop {
        screen::draw(&grid);
        grid.next_gen();

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
