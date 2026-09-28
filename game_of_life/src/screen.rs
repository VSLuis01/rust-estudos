use std::io;

use crossterm::style::Print;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};

use crate::grid::Grid;

const ALIVE: &str = "██";
const DEAD: &str = "  ";

pub fn grid_size() -> (usize, usize) {
    let (columns, rows) = terminal::size().unwrap();
    (columns as usize / 2, rows as usize - 1)
}

pub fn setup() {
    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen, cursor::Hide).unwrap();
}

pub fn restore() {
    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}

pub fn draw(grid: &Grid) {
    let mut output = String::new();

    for row in grid.cells().chunks(grid.width()) {
        for alive in row {
            output.push_str(if *alive { ALIVE } else { DEAD });
        }
        output.push_str("\r\n");
    }

    execute!(io::stdout(), cursor::MoveTo(0, 0), Print(output)).unwrap();
}
