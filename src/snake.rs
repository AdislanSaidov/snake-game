use std::collections::VecDeque;

use graphics::types::ColorComponent;
use piston_window::{Context, G2d};

use crate::direction::Direction;
use crate::draw_utils::draw_square;
use crate::food::Food;
use crate::game::{CELL_SIZE, END_CELL_IDX, START_CELL_IDX};
use crate::map::Map;
use crate::point::Point;

pub struct Snake {
    pub x: f64,
    pub y: f64,
    pub coords: VecDeque<Point>,
    pub direction: Direction,
    pub body_color: [ColorComponent; 4],
    pub stroke_color: [ColorComponent; 4],
    pub is_new_direction_handled: bool
}

impl Snake {
    pub fn new(
        x: f64,
        y: f64,
        direction: Direction,
        coords: VecDeque<Point>,
        body_color: [ColorComponent; 4],
        stroke_color: [ColorComponent; 4],
    ) -> Self {
        Snake {
            x,
            y,
            coords,
            direction,
            body_color,
            stroke_color,
            is_new_direction_handled: true
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        for (x, y) in &self.coords {
            draw_square(
                context,
                graphics,
                self.body_color,
                x * CELL_SIZE,
                y * CELL_SIZE,
                CELL_SIZE,
                Option::Some(self.stroke_color),
            );
        }
        let (head_x, head_y) = self.coords[0];

        draw_square(
            context,
            graphics,
            self.stroke_color,
            head_x * CELL_SIZE + 6,
            head_y * CELL_SIZE + 6,
            CELL_SIZE - 12,
            Option::None,
        );
    }

    pub fn collides_with_food(&self, food: &Food) -> bool {
        let (food_x, food_y) = food.coords;
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        return new_x == food_x && new_y == food_y;
    }

    fn collides_with_walls_or_himself(&self, map: &Map) -> bool {
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        let point = (new_x, new_y);
        return self.coords.contains(&point) || map.coords.contains(&point);
    }

    fn handle_off_screen_movement(&mut self) {
        let new_x = self.x.round();
        let new_y = self.y.round();
        let start_cell_idx = START_CELL_IDX as f64;
        let end_cell_idx = END_CELL_IDX as f64;

        if new_x < start_cell_idx {
            self.x = end_cell_idx;
        } else if new_x > end_cell_idx {
            self.x = start_cell_idx;
        }

        if new_y < start_cell_idx {
            self.y = end_cell_idx;
        } else if new_y > end_cell_idx {
            self.y = start_cell_idx;
        }
    }

    pub fn eat_food(&mut self) {
        let last_idx = self.coords.len() - 1;
        let last_item = self.coords[last_idx];
        self.coords.push_back((last_item.0, last_item.1))
    }

    fn move_one_step(&mut self) {
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        let point = (new_x, new_y);
        self.coords.push_front(point);
        self.coords.pop_back();
        self.is_new_direction_handled = true;
    }

    pub fn change_direction(&mut self, new_direction: Direction, opposite_direction: Direction) -> bool {
        if !self.is_new_direction_handled || self.direction == opposite_direction {
            return false;
        }
        self.direction = new_direction;
        self.is_new_direction_handled = false;

        return true;
    }

    pub fn handle_movement(&mut self, v: f64, map: &Map) -> bool {
        match self.direction {
            Direction::LEFT => {
                let new_value = self.x - v;
                let old_value = self.x;
                self.x = new_value;
                if new_value.round() == old_value.round() {
                    return false;
                }
            }
            Direction::RIGHT => {
                let new_value = self.x + v;
                let old_value = self.x;
                self.x = new_value;
                if new_value.round() == old_value.round() {
                    return false;
                }
            }
            Direction::TOP => {
                let new_value = self.y - v;
                let old_value = self.y;
                self.y = new_value;
                if new_value.round() == old_value.round() {
                    return false;
                }
            }
            Direction::BOTTOM => {
                let new_value = self.y + v;
                let old_value = self.y;
                self.y = new_value;
                if new_value.round() == old_value.round() {
                    return false;
                }
            }
        }

        self.handle_off_screen_movement();

        if self.collides_with_walls_or_himself(&map) {
            return true;
        }
        self.move_one_step();

        return false;
    }
}
