use graphics::{CharacterCache, Text};
use graphics::types::FontSize;
use piston_window::{Context, G2d, Glyphs};
use piston_window::*;
use piston_window::types::ColorComponent;

use crate::draw_utils::draw_rect;
use crate::point::Point;

pub struct GameButton {
    coords: Point,
    width: i32,
    height: i32,
    text: String,
    text_color: [ColorComponent; 4],
    font_size: FontSize,
    background_color: [ColorComponent; 4],
}

impl GameButton {
    pub fn new(
        coords: Point,
        width: i32,
        height: i32,
        text: String,
        text_color: [ColorComponent; 4],
        font_size: FontSize,
        background_color: [ColorComponent; 4],
    ) -> GameButton {
        GameButton {
            coords,
            width,
            height,
            text,
            text_color,
            font_size,
            background_color,
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        let button_start_x = self.coords.0 as f64;
        let button_start_y = self.coords.1 as f64;
        let button_width = self.width as f64;
        let button_height = self.height as f64;
        let button_end_y = button_start_y + button_height;

        draw_rect(
            context,
            graphics,
            self.background_color,
            self.coords,
            self.width,
            self.height,
            Option::Some([1.0, 0.0, 0.0, 1.0]),
        );

        let text_width = glyphs.width(self.font_size, &self.text).ok().unwrap() as f64;
        let free_space_width_in_button = button_width - text_width;
        let horizontal_margin = free_space_width_in_button / 2.0;

        let free_space_height_in_button = button_height - self.font_size as f64;
        let vertical_margin = free_space_height_in_button / 2.0;

        let text_start_x = button_start_x + horizontal_margin;
        let start_text_y = button_end_y - vertical_margin;
        let transform = context.transform.trans(text_start_x, start_text_y);

        Text::new_color(self.text_color, self.font_size).draw(
            &self.text,
            glyphs,
            &context.draw_state,
            transform,
            graphics,
        ).unwrap();
    }

    pub fn is_clicked(&self, x: f64, y: f64) -> bool {
        let button_start_x = self.coords.0 as f64;
        let button_start_y = self.coords.1 as f64;
        let button_width = self.width as f64;
        let button_height = self.height as f64;
        let button_end_x = button_start_x + button_width;
        let button_end_y = button_start_y + button_height;

        return x >= button_start_x &&
            x <= button_end_x &&
            y >= button_start_y &&
            y <= button_end_y;
    }
}
