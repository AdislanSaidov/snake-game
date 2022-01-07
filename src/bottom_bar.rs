use graphics::Text;
use piston_window::{Context, G2d, Glyphs};
use piston_window::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const BOTTOM_BAR_HEIGHT: f64 = 40.0;

pub fn draw_bottom_bar(context: Context, graphics: &mut G2d, glyphs: &mut Glyphs, score: i32) {
    let rect = [0.0, 0.0, WINDOW_WIDTH, BOTTOM_BAR_HEIGHT];
    let transform = context.transform.trans(0.0, WINDOW_HEIGHT - BOTTOM_BAR_HEIGHT);
    rectangle(
        // [0.9, 0.7, 0.7, 1.0],
        // [0.7, 0.95, 0.95, 1.0],
        [0.05, 0.05, 0.05, 1.0],
        rect,
        transform,
        graphics,
    );

    let score_font_size = 15;

    let transform = context.transform.trans(10.0, WINDOW_HEIGHT - ((BOTTOM_BAR_HEIGHT - 14.0) / 2.0));

    Text::new_color([1.0, 0.0, 0.0, 1.0], score_font_size).draw(
        format!("SCORE: {}", score).as_str(),
        glyphs,
        &context.draw_state,
        transform,
        graphics,
    ).unwrap_or_else(|err| println!("{:?}", err));
}