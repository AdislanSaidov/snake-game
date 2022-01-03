use piston_window::{Context, G2d};
use piston_window::types::ColorComponent;

use crate::game::CELL_SIZE;
use crate::point::Point;
use crate::draw_utils::draw_square;

const FOOD_COLOR: [ColorComponent; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct Food{
    pub coords: Point
}

impl Food {
    pub fn new(coords: Point) -> Food {
        Food{ coords }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d,) {
        let (x, y) = self.coords;
        draw_square(
            context,
            graphics,
            FOOD_COLOR,
            x * CELL_SIZE,
            y * CELL_SIZE,
            CELL_SIZE,
            Option::None
        );
    }
}