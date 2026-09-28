use std::io;
use std::time::Duration;

use crossterm::style::Print;
use crossterm::terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};

use crate::grid::Grid;

const ALIVE: &str = "██";
const DEAD: &str = "  ";

#[derive(Default)]
pub struct Timings {
    pub next_gen: Duration,
    pub draw: Duration,
}

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

pub fn draw(grid: &Grid, timings: &Timings) {
    let mut output = String::new();

    for row in grid.cells().chunks(grid.width()) {
        for alive in row {
            output.push_str(if *alive { ALIVE } else { DEAD });
        }
        output.push_str("\r\n");
    }

    output.push_str(&status_bar(grid, timings));

    execute!(
        io::stdout(),
        cursor::MoveTo(0, 0),
        Print(output),
        terminal::Clear(ClearType::UntilNewLine)
    )
    .unwrap();
}

fn status_bar(grid: &Grid, timings: &Timings) -> String {
    let status = format!(
        " geração {} | população {} | grade {}x{} | cálculo {:.1?} | desenho {:.1?} | Esc sai",
        grid.generation(),
        grid.population(),
        grid.width(),
        grid.height(),
        timings.next_gen,
        timings.draw,
    );

    let columns = grid.width() * ALIVE.chars().count();
    status.chars().take(columns).collect()
}
