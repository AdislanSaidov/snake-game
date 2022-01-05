use std::collections::VecDeque;


use crate::direction::Direction;
use crate::point::Point;
use graphics::types::ColorComponent;
use crate::snake::Snake;

pub struct SnakeConfig{
    x: f64,
    y: f64,
    coords: VecDeque<Point>,
    direction: Direction,
    body_color: [ColorComponent; 4],
    stroke_color: [ColorComponent; 4]
}

impl SnakeConfig {
    pub fn from_snake(snake: &Snake) -> SnakeConfig {
        SnakeConfig {
            x: snake.x,
            y: snake.y,
            coords: snake.coords.clone(),
            direction: snake.direction.clone(),
            body_color: snake.body_color,
            stroke_color: snake.stroke_color
        }
    }

    pub fn to_snake(&self) -> Snake {
        Snake {
            x: self.x,
            y: self.y,
            coords: self.coords.clone(),
            direction: self.direction.clone(),
            body_color: self.body_color,
            stroke_color: self.stroke_color
        }
    }
}

