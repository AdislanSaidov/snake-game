use graphics::{Context, rectangle};
use graphics::types::ColorComponent;
use piston_window::*;
use piston_window::G2d;

pub type Color = [ColorComponent; 4];

pub fn draw_square(
    context: Context,
    graphics: &mut G2d,
    color: Color,
    x: f64,
    y: f64,
    size: f64,
    stroke_color: Option<Color>,
) {
    draw_rect(context, graphics, color, x, y, size, size, stroke_color);
}

pub fn draw_rect(
    context: Context,
    graphics: &mut G2d,
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    stroke_color: Option<Color>,
) {
    let square = [0.0, 0.0, width, height];
    let transform = context.transform.trans(x, y);
    rectangle(
        color,
        square,
        transform,
        graphics,
    );
    match stroke_color {
        Option::Some(color) => {
            let line_radius = 0.4;
            line(color, line_radius, [0.0, 0.0, width, 0.0], transform, graphics); // top
            line(color, line_radius, [0.0, 0.0, 0.0, height], transform, graphics); // left
            line(color, line_radius, [0.0, height, width, height], transform, graphics); // bottom
            line(color, line_radius, [width, 0.0, width, height], transform, graphics); // right
        }
        Option::None => {}
    }
}

