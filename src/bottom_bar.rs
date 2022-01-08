use graphics::Text;
use piston_window::{Context, G2d, Glyphs};
use piston_window::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::draw_utils::Color;
use crate::game_button::GameButton;

const BOTTOM_BAR_HEIGHT: f64 = 40.0;

pub struct BottomBar {
    width: f64,
    height: f64,
    button: GameButton,
    background_color: Color
}

impl BottomBar {
    pub fn new(
        width: f64,
        height: f64,
        background_color: Color
    ) -> Self {

        let button_height = 32;
        let button_width = 120;
        let button_end_margin = 10;
        let game_area_height = (WINDOW_HEIGHT - BOTTOM_BAR_HEIGHT) as i32;
        let button_x = WINDOW_WIDTH as i32 - button_width - button_end_margin;
        let button_y = game_area_height + (WINDOW_HEIGHT as i32 - (game_area_height + button_height)) / 2;
        let button = GameButton::new(
            (button_x, button_y),
            button_width,
            button_height,
            "New game".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            8,
            [0.0, 0.0, 0.0, 1.0],
        );

        BottomBar {
            width,
            height,
            button,
            background_color
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs, score: i32) {
        let rect = [0.0, 0.0, self.width, self.height];
        let transform = context.transform.trans(0.0, WINDOW_HEIGHT - self.height);
        rectangle(
            self.background_color,
            rect,
            transform,
            graphics,
        );

        let score_font_size = 15;

        let transform = context.transform.trans(10.0, WINDOW_HEIGHT - ((self.height - 14.0) / 2.0));

        Text::new_color([1.0, 0.0, 0.0, 1.0], score_font_size).draw(
            format!("SCORE: {}", score).as_str(),
            glyphs,
            &context.draw_state,
            transform,
            graphics,
        ).unwrap_or_else(|err| println!("{:?}", err));

        self.button.draw(context, graphics, glyphs);
    }

    pub fn is_button_clicked(&self, x: f64, y: f64) -> bool {
        self.button.is_clicked(x, y)
    }
}


