use piston_window::{Context, G2d};

use crate::game::CELL_SIZE;
use crate::point::Point;
use crate::draw_utils::draw_square;
use graphics::types::ColorComponent;

pub struct Map {
    pub coords: Vec<Point>,
    background_color: [ColorComponent; 4],
    stroke_color: [ColorComponent; 4],
}

impl Map {
    pub fn new(
        coords: Vec<Point>,
        background_color: [ColorComponent; 4],
        stroke_color: [ColorComponent; 4],
    ) -> Map {
        Map {
            coords,
            background_color,
            stroke_color,
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        for (x, y) in &self.coords {
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
