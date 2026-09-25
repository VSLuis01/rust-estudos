extern crate piston_window;
extern crate rand;

use crate::draw::to_pixel_u32;
use crate::game::Game;
use piston_window::graphics::clear;
use piston_window::graphics::types::Color;
use piston_window::{Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

mod draw;
mod game;
mod snake;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_pixel_u32(width), to_pixel_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |context, graphics, _| {
            clear(BACK_COLOR, graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
