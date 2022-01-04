use std::collections::VecDeque;

use piston_window::{Context, G2d};

use crate::direction::Direction;
use crate::food::Food;
use crate::game::{CELL_SIZE, END_CELL_IDX, SNAKE_COLOR, START_CELL_IDX};
use crate::point::Point;
use crate::wall::Walls;
use crate::draw_utils::draw_square;

pub struct Snake{
    pub x: f64,
    pub y: f64,
    pub coords: VecDeque<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(
        x: f64,
        y: f64,
        direction: Direction,
        coords: VecDeque<Point>
    ) -> Snake {
        Snake{
            x,
            y,
            coords,
            direction
        }
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d,) {
        for (x, y) in &self.coords {
            draw_square(
                context,
                graphics,
                SNAKE_COLOR,
                x * CELL_SIZE,
                y * CELL_SIZE,
                CELL_SIZE,
                Option::Some([0.9, 0.0, 0.0, 1.0]),
            );
        }
        let (head_x, head_y) = self.coords[0];

        draw_square(
            context,
            graphics,
            [1.0, 0.0, 0.0, 1.0],
            head_x * CELL_SIZE + 6,
            head_y * CELL_SIZE + 6,
            CELL_SIZE - 12,
            Option::None
        );
    }

    pub fn collides_with_food(&self, food: &Food) -> bool {
        let (food_x, food_y) = food.coords;
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        return  new_x == food_x && new_y == food_y
    }

    pub fn collides_with_walls_or_himself(&self, walls: &Walls) -> bool {
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        let point = (new_x, new_y);
        return self.coords.contains(&point) || walls.coords.contains(&point);
    }

    pub fn handle_off_screen_movement(&mut self) {
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
        let last_item = self.coords.get(last_idx).unwrap();
        self.coords.push_back((last_item.0, last_item.1))
    }

    pub fn move_one_step(&mut self) {
        let new_x = self.x.round() as i32;
        let new_y = self.y.round() as i32;
        let point = (new_x, new_y);
        self.coords.push_front(point);
        self.coords.pop_back();
    }

    pub fn change_direction(&mut self, new_direction: Direction, opposite_direction: Direction) -> bool {
        if self.direction == opposite_direction {
            return false;
        }
        self.direction = new_direction;
        return true;
    }

    pub fn handle_movement(&mut self, v: f64, walls: &Walls) -> bool {
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

        if self.collides_with_walls_or_himself(&walls) {
            return true;
        }
        self.move_one_step();

        return false;
    }

}
