extern crate piston_window;

use std::collections::VecDeque;

use graphics::types::ColorComponent;
use piston_window::*;
use rand::Rng;

use crate::bottom_bar::draw_bottom_bar;
use crate::direction::Direction;
use crate::food::Food;
use crate::point::Point;
use crate::snake::Snake;
use crate::wall::Walls;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::state::State;

pub const SNAKE_COLOR: [ColorComponent; 4] = [0.0, 0.0, 0.0, 1.0];
pub const CELL_SIZE: i32 = 20;
pub const END_CELL_IDX: i32 = 29;
pub const START_CELL_IDX: i32 = 0;


pub struct Game {
    food: Food,
    should_restart_level: bool,
    score: i32,
    glyphs: Glyphs,
    snake: Snake,
    walls: Walls,
    state: State
}

fn generate_food(snake_coords: &VecDeque<Point>, wall_coords: &Vec<Point>) -> Food {
    let mut rng = rand::thread_rng();

    let mut free_coords: Vec<Point> = Vec::new();
    for x in 0..30 {
        for y in 0..30 {
            let point = (x, y);
            if !snake_coords.contains(&point) && !wall_coords.contains(&point) {
                free_coords.push(point);
            }
        }
    }

    let point_idx = rng.gen_range(0..free_coords.len());
    return Food::new(free_coords.remove(point_idx));
}

fn create_walls() -> Vec<Point> {
    // return vec![(0,0)];
    // return vec![];
    return vec![
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
        (0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (0, 16), (0, 17), (0, 18), (0, 19),
        (0, 20), (0, 21), (0, 22), (0, 23), (0, 24), (0, 25), (0, 26), (0, 27), (0, 28), (0, 29),
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0),
        (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0), (20, 0),
        (21, 0), (22, 0), (23, 0), (24, 0), (25, 0), (26, 0), (27, 0), (28, 0), (29, 0),
        (1, 29), (2, 29), (3, 29), (4, 29), (5, 29), (6, 29), (7, 29), (8, 29), (9, 29), (10, 29),
        (11, 29), (12, 29), (13, 29), (14, 29), (15, 29), (16, 29), (17, 29), (18, 29), (19, 29), (20, 29),
        (21, 29), (22, 29), (23, 29), (24, 29), (25, 29), (26, 29), (27, 29), (28, 29), (29, 29),
        (29, 1), (29, 2), (29, 3), (29, 4), (29, 5), (29, 6), (29, 7), (29, 8), (29, 9),
        (29, 10), (29, 11), (29, 12), (29, 13), (29, 14), (29, 15), (29, 16), (29, 17), (29, 18), (29, 19),
        (29, 20), (29, 21), (29, 22), (29, 23), (29, 24), (29, 25), (29, 26), (29, 27), (29, 28),

        // (15,15), (16,15), (16,16), (14,14),(14,15),(14,16),(15,14),(15,16)
    ];
}

fn create_snake() -> VecDeque<Point> {
    let mut deque: VecDeque<Point> = VecDeque::new();
    (15..18).into_iter().for_each(|i| {
        deque.push_back((15, i));
    });
    return deque;
}

impl Game {
    pub fn new(glyphs: Glyphs) -> Game {
        let wall_coords: Vec<Point> = create_walls();
        let deque = create_snake();
        let food = generate_food(&deque, &wall_coords);

        Game {
            snake: Snake::new(15.0, 15.0, Direction::TOP, deque),
            walls: Walls::new(wall_coords),
            food,
            should_restart_level: false,
            score: 0,
            glyphs,
            state: State::NotStarted
        }
    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {
        if self.should_restart_level {
            self.should_restart_level = false;
            self.state = State::NotStarted;
            self.score = 0;

            let deque = create_snake();
            let food = generate_food(&deque, &self.walls.coords);
            self.snake = Snake::new(15.0, 15.0, Direction::TOP,deque);
            self.food = food;

            return;
        }
        if self.state != State::Playing {
            return;
        }
        // 5 cells per second
        let v = 5.0 * upd.dt;
        let is_game_over = self.snake.handle_movement(v, &self.walls);

        if is_game_over {
            self.state = State::GameOver;
            return;
        }

        if self.snake.collides_with_food(&self.food) {
            self.snake.eat_food();
            self.score += 1;
            self.food = generate_food(&self.snake.coords, &self.walls.coords);
        }
    }

    pub fn on_draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            self.walls.draw(context, graphics);
            self.snake.draw(context, graphics);
            self.food.draw(context, graphics);

            match self.state {
                State::GameOver => {
                    self.draw_message(&context, graphics, "GAME OVER!");
                }
                State::Paused => {
                    self.draw_message(&context, graphics, "Pause");
                }
                _ => {}
            }

            draw_bottom_bar(context, graphics, &mut self.glyphs, self.score);

            self.glyphs.factory.encoder.flush(device);
        });
    }

    fn draw_message(&mut self, context: &Context, graphics: &mut G2d, text: &str) {
        let font_size = 24;
        let text_width = self.glyphs.width(font_size, text).ok().unwrap();
        let x = WINDOW_WIDTH / 2. - text_width / 2.;
        let y = WINDOW_HEIGHT / 2.;

        let transform = context.transform.trans(x, y);

        Text::new_color([0.0, 1.0, 0.0, 1.0], font_size).draw(
            text,
            &mut self.glyphs,
            &context.draw_state,
            transform,
            graphics,
        ).unwrap();
    }

    pub fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(button_args) => match button_args.state {
                ButtonState::Press => {
                    if self.state == State::GameOver {
                        self.should_restart_level = true;
                        return;
                    }
                
                    match button_args.button {
                        Button::Keyboard(Key::Up) => {
                            self.verify_moving();
                            self.snake.change_direction(Direction::TOP, Direction::BOTTOM);
                        }
                        Button::Keyboard(Key::Down) => {
                            self.verify_moving();
                            self.snake.change_direction(Direction::BOTTOM, Direction::TOP);
                        }
                        Button::Keyboard(Key::Left) => {
                            self.verify_moving();
                            self.snake.change_direction(Direction::LEFT, Direction::RIGHT);
                        }
                        Button::Keyboard(Key::Right) => {
                            self.verify_moving();
                            self.snake.change_direction(Direction::RIGHT, Direction::LEFT);
                        }
                        Button::Keyboard(Key::P) => {
                            self.handle_pause()
                        }
                        Button::Keyboard(Key::Space) => {
                            self.handle_pause()
                        }
                        _ => (),
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_pause(&mut self) {
        match self.state {
            State::Paused => {
                self.state = State::Playing
            }
            _ => {
                self.state = State::Paused
            }
        }
    }

    fn verify_moving(&mut self){
        if self.state == State::NotStarted || self.state == State::Paused {
            self.state = State::Playing
        }
    }
}
