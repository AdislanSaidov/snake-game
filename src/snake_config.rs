use std::collections::VecDeque;

use graphics::types::ColorComponent;

use crate::direction::Direction;
use crate::point::Point;
use crate::snake::Snake;

pub struct SnakeConfig {
    x: f64,
    y: f64,
    coords: VecDeque<Point>,
    direction: Direction,
    body_color: [ColorComponent; 4],
    stroke_color: [ColorComponent; 4],
}

impl SnakeConfig {
    pub fn from_snake(snake: &Snake) -> SnakeConfig {
        SnakeConfig {
            x: snake.x,
            y: snake.y,
            coords: snake.coords.clone(),
            direction: snake.direction.clone(),
            body_color: snake.body_color,
            stroke_color: snake.stroke_color,
        }
    }

    pub fn to_snake(&self) -> Snake {
        Snake::new(
            self.x,
            self.y,
            self.direction.clone(),
            self.coords.clone(),
            self.body_color,
            self.stroke_color
        )
    }
}

