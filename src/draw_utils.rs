use graphics::{Context, rectangle};
use graphics::types::ColorComponent;
use piston_window::*;
use piston_window::G2d;

use crate::point::Point;

pub fn draw_square(
    context: Context,
    graphics: &mut G2d,
    color: [ColorComponent; 4],
    x: i32,
    y: i32,
    size: i32,
    stroke_color: Option<[ColorComponent; 4]>,
) {
    draw_rect(context, graphics, color, (x, y), size, size, stroke_color);
}

pub fn draw_rect(
    context: Context,
    graphics: &mut G2d,
    color: [ColorComponent; 4],
    point: Point,
    width: i32,
    height: i32,
    stroke_color: Option<[ColorComponent; 4]>,
) {
    let new_width = width as f64;
    let new_height = height as f64;
    let square = [0.0, 0.0, new_width, new_height];
    let transform = context.transform.trans(point.0 as f64, point.1 as f64);
    rectangle(
        color,
        square,
        transform,
        graphics,
    );
    match stroke_color {
        Option::Some(color) => {
            let line_radius = 0.4;
            line(color, line_radius, [0.0, 0.0, new_width, 0.0], transform, graphics); // top
            line(color, line_radius, [0.0, 0.0, 0.0, new_height], transform, graphics); // left
            line(color, line_radius, [0.0, new_height, new_width, new_height], transform, graphics); // bottom
            line(color, line_radius, [new_width, 0.0, new_width, new_height], transform, graphics); // right
        }
        Option::None => {}
    }
}
