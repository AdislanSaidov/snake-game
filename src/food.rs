use piston_window::{Context, Event, G2d, PistonWindow, RenderArgs};
use piston_window::types::ColorComponent;

use crate::Game;
use crate::point::Point;
use crate::game::CELL_SIZE;

const FOOD_COLOR: [ColorComponent; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct Food{
    pub coords: Point
}

impl Food {
    pub fn new(coords: Point) -> Food {
        Food{ coords }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d,) {
        Game::draw_square(
            context,
            graphics,
            FOOD_COLOR,
            self.coords.x * CELL_SIZE,
            self.coords.y * CELL_SIZE,
        );
    }
}