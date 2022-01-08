use std::collections::VecDeque;

use crate::direction::Direction;
use crate::draw_utils::Color;
use crate::point::Point;
use crate::snake::Snake;

pub struct SnakeConfig {
    coords: VecDeque<Point>,
    direction: Direction,
    body_color: Color,
    stroke_color: Color,
}

impl SnakeConfig {
    pub fn from_snake(snake: &Snake) -> SnakeConfig {
        SnakeConfig {
            coords: snake.coords.clone(),
            direction: snake.direction.clone(),
            body_color: snake.body_color,
            stroke_color: snake.stroke_color,
        }
    }

    pub fn to_snake(&self) -> Snake {
        Snake::new(
            self.direction.clone(),
            self.coords.clone(),
            self.body_color,
            self.stroke_color
        )
    }
}

