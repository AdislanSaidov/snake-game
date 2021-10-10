extern crate piston_window;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use piston_window::*;
use rand::Rng;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 600.0;

const CELL_SIZE: f64 = 20.0;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    NONE,
}

struct Point {
    x: i32,
    y: i32,
}

struct Game {
    x_delta: f64,
    y_delta: f64,
    x: i32,
    y: i32,
    direction: Direction,
    snake_coords: VecDeque<Point>,
    food_point: Point,
}

fn generate_food(snake_coords: &VecDeque<Point>) -> Point {
    let mut rng = rand::thread_rng();

    let mut free_coords: Vec<Point> = Vec::new();
    (0..30).into_iter().for_each(|i| {
        (0..30).into_iter().for_each(|k| {
            for p in snake_coords {
                // println!("x: {} y: {} snake: x: {} y: {}", i, k, p.x, p.y);
                if i != p.x && k != p.y {
                    free_coords.push(Point { x: i, y: k });
                }
            }
        });
    });


    let point_idx = rng.gen_range(0..free_coords.len());
    return free_coords.remove(point_idx);
}

impl Game {
    fn new() -> Game {
        let mut deque: VecDeque<Point> = VecDeque::new();
        deque.push_back(Point { x: 2, y: 2 });
        deque.push_back(Point { x: 2, y: 3 });
        deque.push_back(Point { x: 2, y: 4 });
        // (3..980).into_iter().for_each( |i| {
        //     deque.push_back(Point { x: 2, y: i });
        // });
        let food_point = generate_food(&deque);
        Game {
            x_delta: 1.0,
            y_delta: 1.0,
            x: 1,
            y: 1,
            direction: Direction::NONE,
            snake_coords: deque,
            food_point,
        }
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
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

            // println!("{}", upd.dt);
            // println!("{}", self.snake_coords.len().to_string());
        }

        if self.x == self.food_point.x && self.y == self.food_point.y {
            self.food_point = generate_food(&self.snake_coords);
            let last_idx = self.snake_coords.len() - 1;
            let last_item = self.snake_coords.get(last_idx).unwrap();
            self.snake_coords.push_back(Point { x: last_item.x, y: last_item.y })
        }
    }

    fn on_draw(&mut self, ren: &RenderArgs, window: &mut PistonWindow, event: &Event) {
        let window_dimensions = ren.draw_size;
        let width = window_dimensions[0];
        let height = window_dimensions[1];

        window.draw_2d(event, |c, g, _device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let red = [1.0, 0.0, 0.0, 1.0];
            for point in &self.snake_coords {
                // println!("LOOP {} {}", point.x, point.y);
                let square = rectangle::square(0.0, 0.0, CELL_SIZE);
                let center = c.transform.trans((point.x as f64) * CELL_SIZE, (point.y as f64) * CELL_SIZE);
                rectangle(
                    red,
                    square,
                    center,
                    g,
                );
            }
            let green = [0.0, 1.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, CELL_SIZE);
            let center = c.transform.trans(
                (self.food_point.x as f64) * CELL_SIZE,
                (self.food_point.y as f64) * CELL_SIZE,
            );
            rectangle(
                green,
                square,
                center,
                g,
            );
        });
    }

    fn on_input(&mut self, inp: &Input) {
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

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake game", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let mut game = Game::new();

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Update(ref upd)) => game.on_update(upd),
            Event::Loop(Loop::Render(ref ren)) => game.on_draw(ren, &mut window, &event),
            Event::Input(ref inp, _) => game.on_input(inp),
            _ => {}
        }
    }
}