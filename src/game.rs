extern crate piston_window;

use std::collections::VecDeque;

use graphics::types::ColorComponent;
use piston_window::*;
use rand::Rng;

use crate::bottom_bar::{draw_bottom_bar, BOTTOM_BAR_HEIGHT};
use crate::direction::Direction;
use crate::food::Food;
use crate::point::Point;
use crate::snake::Snake;
use crate::map::Map;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::state::State;
use crate::game_button::GameButton;
use crate::resources::{create_stuff, generate_food};
use crate::snake_config::SnakeConfig;

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
    walls: Map,
    state: State,
    button: GameButton,
    last_mouse_pos: Option<[f64; 2]>,
    snake_config: SnakeConfig
}


impl Game {
    pub fn new(glyphs: Glyphs, snake: Snake, map: Map, food: Food) -> Self {

        let snake_config = SnakeConfig::from_snake(&snake);

        let button_height = 32;
        let button_width = 120;
        let button_end_margin = 10;
        let game_area_height = (WINDOW_HEIGHT - BOTTOM_BAR_HEIGHT) as i32;
        let button_x = WINDOW_WIDTH as i32 - button_width - button_end_margin;
        let button_y = game_area_height + (WINDOW_HEIGHT as i32 - (game_area_height + button_height)) / 2;
        let button = GameButton::new(
            (button_x, button_y),
            button_width,
            button_height,
            "New game".to_string(),
            [1.0, 0.0, 0.0, 1.0],
            8,
            [0.0, 0.0, 0.0, 1.0],
            || {

                println!("CLICK");
            }
        );

        Game {
            snake,
            walls: map,
            food,
            should_restart_level: false,
            score: 0,
            glyphs,
            state: State::NotStarted,
            button,
            last_mouse_pos: Option::None,
            snake_config
        }
    }

    fn new_game(&mut self) {
        let (snake, map, food) = create_stuff();

    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {
        if self.should_restart_level {
            self.should_restart_level = false;
            self.state = State::NotStarted;
            self.score = 0;

            self.snake = self.snake_config.to_snake();
            let food = generate_food(&self.snake.coords, &self.walls.coords);
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
            self.button.draw(context, graphics, &mut self.glyphs);

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
        ).unwrap_or_else(|err| println!("{:?}", err));
    }

    pub fn on_mouse_input(&mut self, pos: [f64; 2]){
        self.last_mouse_pos = Option::Some(pos);
    }

    pub fn on_input(&mut self, inp: &Input, event: &Event) {
        match inp {
            Input::Button(button_args) => match button_args.state {
                ButtonState::Press => {
                    match button_args.button {
                        Button::Mouse(_button) => {}
                        _ => {
                            if self.state == State::GameOver {
                                self.should_restart_level = true;
                                return;
                            }
                        }
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
                        Button::Mouse(MouseButton::Left) => {
                            self.last_mouse_pos.map(|pos|{
                                let (x, y) = (pos[0], pos[1]);
                                self.button.on_click_event(x, y)
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
