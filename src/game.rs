extern crate piston_window;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use piston_window::*;
use rand::Rng;
use graphics::types::ColorComponent;
use crate::food::Food;
use crate::point::Point;
use crate::direction::Direction;

const SNAKE_COLOR: [ColorComponent; 4] = [1.0, 0.0, 0.0, 1.0];
pub const CELL_SIZE: i32 = 20;

pub struct Game {
    x_delta: f64,
    y_delta: f64,
    x: i32,
    y: i32,
    direction: Direction,
    snake_coords: VecDeque<Point>,
    food: Food
}

fn generate_food(snake_coords: &VecDeque<Point>) -> Food {
    let mut rng = rand::thread_rng();

    let mut free_coords: Vec<Point> = Vec::new();
    for x in 0..30 {
        for y in 0..30 {
            for p in snake_coords {
                if x != p.x && y != p.y {
                    free_coords.push(Point { x, y });
                }
            }
        }
    }

    let point_idx = rng.gen_range(0..free_coords.len());
    return Food::new(free_coords.remove(point_idx));
}

impl Game {
    pub fn new() -> Game {
        let mut deque: VecDeque<Point> = VecDeque::new();
        deque.push_back(Point { x: 2, y: 2 });
        deque.push_back(Point { x: 2, y: 3 });
        deque.push_back(Point { x: 2, y: 4 });
        (5..10).into_iter().for_each(|i| {
            deque.push_back(Point { x: 2, y: i });
        });
        let food = generate_food(&deque);
        Game {
            x_delta: 1.0,
            y_delta: 1.0,
            x: 1,
            y: 1,
            direction: Direction::NONE,
            snake_coords: deque,
            food
        }
    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {
        let v = 5.0 * upd.dt;
        match self.direction {
            Direction::LEFT => {
                self.x_delta -= v;
            }
            Direction::RIGHT => {
                self.x_delta += v;
            }
            Direction::TOP => {
                self.y_delta -= v;
            }
            Direction::BOTTOM => {
                self.y_delta += v;
            }
            Direction::NONE => {}
        }

        if self.direction != Direction::NONE {
            let old_x = self.x;
            let old_y = self.y;
            self.x = self.x_delta.round() as i32;
            self.y = self.y_delta.round() as i32;
            if old_x == self.x && old_y == self.y {
                return;
            }
            self.snake_coords.push_front(Point { x: self.x, y: self.y });
            self.snake_coords.pop_back();
        }

        if self.x == self.food.coords.x && self.y == self.food.coords.y {
            self.food = generate_food(&self.snake_coords);
            let last_idx = self.snake_coords.len() - 1;
            let last_item = self.snake_coords.get(last_idx).unwrap();
            self.snake_coords.push_back(Point { x: last_item.x, y: last_item.y })
        }
    }

    pub fn on_draw(&mut self, ren: &RenderArgs, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            for point in &self.snake_coords {
                Game::draw_square(
                    context,
                    graphics,
                    SNAKE_COLOR,
                    point.x * CELL_SIZE,
                    point.y * CELL_SIZE,
                );
            }

            self.food.draw(context, graphics);
        });
    }

    pub fn draw_square(context: Context, graphics: &mut G2d, color: [ColorComponent; 4], x: i32, y: i32) {
        let square = rectangle::square(0.0, 0.0, CELL_SIZE as f64);
        let center = context.transform.trans(x as f64, y as f64);
        rectangle(
            color,
            square,
            center,
            graphics,
        );
    }

    pub fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(but) => match but.state {
                ButtonState::Press => match but.button {
                    Button::Keyboard(Key::Up) => {
                        self.change_direction(Direction::TOP, Direction::BOTTOM);
                    }
                    Button::Keyboard(Key::Down) => {
                        self.change_direction(Direction::BOTTOM, Direction::TOP);
                    }
                    Button::Keyboard(Key::Left) => {
                        self.change_direction(Direction::LEFT, Direction::RIGHT);
                    }
                    Button::Keyboard(Key::Right) => {
                        self.change_direction(Direction::RIGHT, Direction::LEFT);
                    }
                    _ => (),
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn change_direction(&mut self, new_direction: Direction, opposite_direction: Direction) {
        if self.direction == opposite_direction {
            return;
        }
        self.direction = new_direction
    }
}