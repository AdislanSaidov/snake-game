use std::collections::VecDeque;

use crate::draw_utils::Color;
use crate::point::Point;
use crate::snake::Snake;

pub struct SnakeConfig {
    coords: VecDeque<Point>,
    body_color: Color,
    stroke_color: Color,
}

impl SnakeConfig {
    pub fn from_snake(snake: &Snake) -> SnakeConfig {
        SnakeConfig {
            coords: snake.coords.clone(),
            body_color: snake.body_color,
            stroke_color: snake.stroke_color,
        }
    }

    pub fn to_snake(&self) -> Snake {
        Snake::new(
            self.coords.clone(),
            self.body_color,
            self.stroke_color
        )
    }
}

