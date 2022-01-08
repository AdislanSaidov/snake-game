extern crate piston_window;

use piston_window::*;

use crate::WINDOW_WIDTH;
use crate::alert::Alert;
use crate::bottom_bar::BottomBar;
use crate::direction::Direction;
use crate::food::Food;
use crate::map::Map;
use crate::resources::{create_stuff, generate_food};
use crate::snake::Snake;
use crate::snake_config::SnakeConfig;
use crate::state::State;

pub const CELL_SIZE: f64 = 20.;
pub const END_CELL_IDX: i32 = 29;
pub const START_CELL_IDX: i32 = 0;


pub struct Game {
    food: Food,
    score: i32,
    glyphs: Glyphs,
    snake: Snake,
    map: Map,
    state: State,
    last_mouse_pos: Option<[f64; 2]>,
    snake_config: SnakeConfig,
    bottom_bar: BottomBar,
    game_over_alert: Alert,
    pause_alert: Alert
}

impl Game {
    pub fn new(glyphs: Glyphs, snake: Snake, map: Map, food: Food) -> Self {
        let snake_config = SnakeConfig::from_snake(&snake);

        Game {
            snake,
            map,
            food,
            score: 0,
            glyphs,
            state: State::NotStarted,
            last_mouse_pos: Option::None,
            snake_config,
            bottom_bar: BottomBar::new(
                WINDOW_WIDTH,
                40.0,
                [0.05, 0.05, 0.05, 1.0]
            ),
            game_over_alert: Alert::new("GAME OVER!".into(), 24, [0.5, 0.5, 0.0, 1.0]),
            pause_alert: Alert::new("Pause".into(), 24, [0.0, 1.0, 0.0, 1.0])
        }
    }

    fn new_game(&mut self) {
        self.state = State::NotStarted;
        let (snake, map, food) = create_stuff();
        self.snake_config = SnakeConfig::from_snake(&snake);
        self.snake = snake;
        self.map = map;
        self.food = food;
        self.score = 0;
    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {
        if self.state != State::Playing {
            return;
        }
        self.food.update(upd.dt);

        let has_collision = self.snake.handle_movement(upd.dt, &self.map);

        if has_collision {
            self.state = State::GameOver;
            return;
        }

        if self.snake.collides_with_food(&self.food) {
            self.eat_food();
        }
    }

    fn eat_food(&mut self) {
        self.snake.eat_food();
        self.score += 1;
        self.food = generate_food(&self.snake.coords, &self.map.coords);
    }

    fn restart(&mut self) {
        self.state = State::NotStarted;
        self.score = 0;
        self.snake = self.snake_config.to_snake();
        self.food = generate_food(&self.snake.coords, &self.map.coords);
    }

    pub fn on_draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            self.map.draw(context, graphics);
            self.snake.draw(context, graphics);
            self.food.draw(context, graphics);

            match self.state {
                State::GameOver => {
                    self.game_over_alert.draw(context, graphics, &mut self.glyphs);
                }
                State::Paused => {
                    self.pause_alert.draw(context, graphics, &mut self.glyphs);
                }
                _ => {}
            }
            self.bottom_bar.draw(context, graphics, &mut self.glyphs, self.score);

            self.glyphs.factory.encoder.flush(device);
        });
    }

    pub fn on_mouse_input(&mut self, pos: [f64; 2]) {
        self.last_mouse_pos = Option::Some(pos);
    }

    pub fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(button_args) => match button_args.state {
                ButtonState::Press => {
                    match button_args.button {
                        Button::Mouse(_button) => {}
                        _ => {
                            if self.state == State::GameOver {
                                self.restart();
                                return;
                            }
                        }
                    }

                    match button_args.button {
                        Button::Keyboard(Key::Up) => {
                            self.play();
                            self.snake.change_direction(Direction::UP);
                        }
                        Button::Keyboard(Key::Down) => {
                            self.play();
                            self.snake.change_direction(Direction::DOWN);
                        }
                        Button::Keyboard(Key::Left) => {
                            self.play();
                            self.snake.change_direction(Direction::LEFT);
                        }
                        Button::Keyboard(Key::Right) => {
                            self.play();
                            self.snake.change_direction(Direction::RIGHT);
                        }
                        Button::Keyboard(Key::N) => {
                            self.new_game();
                        }
                        Button::Keyboard(Key::P) => {
                            self.toggle_paused_state();
                        }
                        Button::Keyboard(Key::R) => {
                            self.restart();
                        }
                        Button::Keyboard(Key::Space) => {
                            self.toggle_paused_state();
                        }
                        Button::Mouse(MouseButton::Left) => {
                            self.last_mouse_pos.map(|pos| {
                                if self.bottom_bar.is_button_clicked(pos[0], pos[1]) {
                                    self.new_game();
                                }
                            });
                        }
                        _ => (),
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn toggle_paused_state(&mut self) {
        match self.state {
            State::Paused => {
                self.state = State::Playing
            }
            _ => {
                self.state = State::Paused
            }
        }
    }

    fn play(&mut self) {
        if self.state == State::NotStarted || self.state == State::Paused {
            self.state = State::Playing
        }
    }
}
