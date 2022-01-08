use piston_window::{G2d, Glyphs};
use graphics::{Text, Context, CharacterCache};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use graphics::types::FontSize;
use piston_window::*;
use crate::draw_utils::Color;

pub struct Alert {
    text: String,
    font_size: FontSize,
    text_color: Color
}

impl Alert {

    pub fn new(
        text: String,
        font_size: FontSize,
        text_color: Color
    ) -> Self {
        Alert{
            text,
            font_size,
            text_color
        }
    }

    pub fn draw(&mut self, context: Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        glyphs.width(self.font_size, &self.text).map(|text_width|{
            let x = WINDOW_WIDTH / 2. - text_width / 2.;
            let y = WINDOW_HEIGHT / 2.;

            let transform = context.transform.trans(x, y);

            Text::new_color(self.text_color, self.font_size).draw(
                &self.text,
                glyphs,
                &context.draw_state,
                transform,
                graphics,
            ).unwrap_or_else(|err| println!("{:?}", err));
        }).ok();
    }
}