use piston_window::{
    graphics::Context, graphics::rectangle, graphics::types::Color, wgpu_graphics::WgpuGraphics,
};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_pixel(game_block: i32) -> f64 {
    (game_block as f64) * BLOCK_SIZE
}

pub fn to_pixel_u32(game_coord: i32) -> u32 {
    to_pixel(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut WgpuGraphics) {
    let gui_x = to_pixel(x);
    let gui_y = to_pixel(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut WgpuGraphics,
) {
    let x = to_pixel(x);
    let y = to_pixel(y);

    rectangle(
        color,
        [x, y, to_pixel(width), to_pixel(height)],
        con.transform,
        g,
    )
}
