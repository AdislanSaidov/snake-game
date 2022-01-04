use graphics::{Context, rectangle};
use piston_window::G2d;
use graphics::types::ColorComponent;
use piston_window::*;

pub fn draw_square(
    context: Context,
    graphics: &mut G2d,
    color: [ColorComponent; 4],
    x: i32,
    y: i32,
    size: i32,
    stroke_color: Option<[ColorComponent; 4]>
) {
    let new_size = size as f64;
    let square = rectangle::square(0.0, 0.0, new_size);
    let transform = context.transform.trans(x as f64, y as f64);
    rectangle(
        color,
        square,
        transform,
        graphics,
    );
    match stroke_color {
        Option::Some(color) => {
            let line_radius = 0.4;
            line(color, line_radius, [0.0, 0.0, 0.0, new_size], transform, graphics);
            line(color, line_radius, [0.0, 0.0, new_size, 0.0], transform, graphics);
            line(color, line_radius, [new_size, 0.0, new_size, new_size], transform, graphics);
            line(color, line_radius, [0.0, new_size, new_size, new_size], transform, graphics);
        }
        Option::None => {}
    }
}
