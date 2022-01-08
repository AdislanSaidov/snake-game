use piston_window::{Context, G2d};

use crate::draw_utils::{Color, draw_square};
use crate::game::CELL_SIZE;
use crate::point::Point;

pub struct Map {
    pub coords: Vec<Point>,
    background_color: Color,
    stroke_color: Color,
}

impl Map {
    pub fn new(
        coords: Vec<Point>,
        background_color: Color,
        stroke_color: Color,
    ) -> Map {
        Map {
            coords,
            background_color,
            stroke_color,
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        for &point in &self.coords {
            let x = point.0 as f64;
            let y = point.1 as f64;

            draw_square(
                context,
                graphics,
                self.background_color,
                x * CELL_SIZE,
                y * CELL_SIZE,
                CELL_SIZE,
                Option::Some(self.stroke_color),
            );
        }
    }
}
