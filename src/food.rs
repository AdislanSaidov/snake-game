use piston_window::{Context, G2d};

use crate::draw_utils::draw_square;
use crate::game::CELL_SIZE;
use crate::point::Point;

pub struct Food {
    pub coords: Point,
    dt: f64
}

impl Food {
    pub fn new(coords: Point) -> Self {
        Food {
            coords,
            dt: 1.0
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.dt += dt;
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        let x = self.coords.0 as f64;
        let y = self.coords.1 as f64;

        let value = self.dt.sin().abs();
        let offset = value * (CELL_SIZE / 2.);

        let color_item = value as f32;
        let color = [color_item, color_item, color_item, 1.0];
        draw_square(
            context,
            graphics,
            color,
            x * CELL_SIZE - offset + CELL_SIZE / 2.,
            y * CELL_SIZE - offset + CELL_SIZE / 2.,
            CELL_SIZE * value,
            Option::None,
        );
    }
}
