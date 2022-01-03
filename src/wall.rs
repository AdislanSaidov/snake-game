
use piston_window::{Context, G2d};

use crate::game::CELL_SIZE;
use crate::point::Point;
use crate::draw_utils::draw_square;

pub struct Walls {
    pub coords: Vec<Point>
}

impl Walls {
    pub fn new(coords: Vec<Point>) -> Walls {
        Walls {
            coords,
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d,) {
        for (x, y) in &self.coords {
            draw_square(
                context,
                graphics,
                [0.67, 0.65, 0.6, 1.0],
                // [0.1, 0.1, 0.1, 1.0],
                x * CELL_SIZE,
                y * CELL_SIZE,
                CELL_SIZE,
                Option::Some([0.5, 0.5, 0.4, 1.0]),
            );
        }
    }

}