extern crate piston_window;

use std::collections::VecDeque;
use std::ops::Index;

use graphics::types::ColorComponent;
use piston_window::*;
use rand::Rng;

use crate::BOTTOM_BAR_HEIGHT;
use crate::direction::Direction;
use crate::food::Food;
use crate::point::Point;
use graphics::ellipse::circle;

const SNAKE_COLOR: [ColorComponent; 4] = [0.0, 0.0, 0.0, 1.0];
pub const CELL_SIZE: i32 = 20;
const END_CELL_IDX: i32 = 29;
const START_CELL_IDX: i32 = 0;


pub struct Game {
    x_delta: f64,
    y_delta: f64,
    x: i32,
    y: i32,
    direction: Direction,
    snake_coords: VecDeque<Point>,
    food: Food,
    is_game_over: bool,
    should_restart_level: bool,
    score: i32,
    wall_coords: Vec<Point>,
    glyphs: Glyphs,
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
            x_delta: 15.0,
            y_delta: 15.0,
            x: 15,
            y: 15,
            direction: Direction::NONE,
            snake_coords: deque,
            food,
            is_game_over: false,
            should_restart_level: false,
            score: 0,
            wall_coords,
            glyphs,
        }
    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {
        if self.should_restart_level {
            self.should_restart_level = false;
            self.direction = Direction::NONE;
            self.is_game_over = false;
            self.x_delta = 15.0;
            self.y_delta = 15.0;
            self.x = 15;
            self.y = 15;
            self.score = 0;

            let deque = create_snake();
            let food = generate_food(&deque, &self.wall_coords);
            self.snake_coords = deque;
            self.food = food;

            return;
        }
        // 5 cells per second
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

            self.handle_off_screen_movement();

            let point = (self.x, self.y);
            println!("{} {}", point.0, point.1);

            let has_collisions = self.handle_collisions(&point);

            if has_collisions {
                return;
            }

            self.snake_coords.push_front(point);
            self.snake_coords.pop_back();
        }
        let (food_x, food_y) = self.food.coords;
        if self.x == food_x && self.y == food_y {
            self.score += 1;
            self.food = generate_food(&self.snake_coords, &self.wall_coords);
            let last_idx = self.snake_coords.len() - 1;
            let last_item = self.snake_coords.get(last_idx).unwrap();
            self.snake_coords.push_back((last_item.0, last_item.1))
        }
    }

    fn handle_collisions(&mut self, point: &Point) -> bool {
        if self.snake_coords.contains(&point) || self.wall_coords.contains(&point) {
            self.is_game_over = true;
            self.direction = Direction::NONE;
            return true;
        }
        return false;
    }

    fn handle_off_screen_movement(&mut self) {
        if self.x < START_CELL_IDX {
            self.x = END_CELL_IDX;
            self.x_delta = END_CELL_IDX as f64;
        } else if self.x > END_CELL_IDX {
            self.x = START_CELL_IDX;
            self.x_delta = START_CELL_IDX as f64;
        }

        if self.y < START_CELL_IDX {
            self.y = END_CELL_IDX;
            self.y_delta = END_CELL_IDX as f64;
        } else if self.y > END_CELL_IDX {
            self.y = START_CELL_IDX;
            self.y_delta = START_CELL_IDX as f64;
        }
    }

    pub fn on_draw(&mut self, ren: &RenderArgs, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            for (x, y) in &self.wall_coords {
                Game::draw_square(
                    context,
                    graphics,
                    [0.67, 0.65, 0.6, 1.0],
                    // [0.1, 0.1, 0.1, 1.0],
                    x * CELL_SIZE,
                    y * CELL_SIZE,
                    CELL_SIZE,
                    Option::Some([0.5, 0.5, 0.4, 1.0]),
                );
            }

            for (x, y) in &self.snake_coords {
                Game::draw_square(
                    context,
                    graphics,
                    SNAKE_COLOR,
                    x * CELL_SIZE,
                    y * CELL_SIZE,
                    CELL_SIZE,
                    Option::Some([0.9, 0.0, 0.0, 1.0]),
                );
            }
            let (head_x, head_y) = self.snake_coords[0];

            Game::draw_square(
                context,
                graphics,
                [1.0, 0.0, 0.0, 1.0],
                head_x * CELL_SIZE + 6,
                head_y * CELL_SIZE + 6,
                CELL_SIZE - 12,
                Option::None
            );


            self.food.draw(context, graphics);

            let window_width = ren.window_size[0];
            let window_height = ren.window_size[1];

            if self.is_game_over {
                let text = "GAME OVER!";
                let font_size = 24;
                let text_width = self.glyphs.width(font_size, text).ok().unwrap();
                let x = window_width / 2. - text_width / 2.;
                let y = window_height / 2.;

                let transform = context.transform.trans(x, y);

                Text::new_color([0.0, 1.0, 0.0, 1.0], font_size).draw(
                    text,
                    &mut self.glyphs,
                    &context.draw_state,
                    transform,
                    graphics,
                ).unwrap();
            }


            let rect = [0.0, 0.0, window_width, BOTTOM_BAR_HEIGHT];
            let transform = context.transform.trans(0.0, window_height - BOTTOM_BAR_HEIGHT);
            rectangle(
                // [0.9, 0.7, 0.7, 1.0],
                // [0.7, 0.95, 0.95, 1.0],
                [0.05, 0.05, 0.05, 1.0],
                rect,
                transform,
                graphics,
            );

            let score_font_size = 15;

            let transform = context.transform.trans(10.0, window_height - ((BOTTOM_BAR_HEIGHT - 14.0) / 2.0));

            Text::new_color([1.0, 0.0, 0.0, 1.0], score_font_size).draw(
                format!("SCORE: {}", self.score).as_str(),
                &mut self.glyphs,
                &context.draw_state,
                transform,
                graphics,
            ).unwrap();

            self.glyphs.factory.encoder.flush(device);
        });
    }

    pub fn draw_square(
        context: Context,
        graphics: &mut G2d,
        color: [ColorComponent; 4],
        x: i32,
        y: i32,
        size: i32,
        stroke_color: Option<[ColorComponent; 4]>
    ) {
        let square = rectangle::square(0.0, 0.0, size as f64);
        let transform = context.transform.trans(x as f64, y as f64);
        rectangle(
            color,
            square,
            transform,
            graphics,
        );
        match stroke_color {
            Option::Some(color) => {
                let line_radius = 0.4;
                line(color, line_radius, [0.0, 0.0, 0.0, size as f64], transform, graphics);
                line(color, line_radius, [0.0, 0.0, size as f64, 0.0], transform, graphics);
                line(color, line_radius, [size as f64, 0.0, size as f64, size as f64], transform, graphics);
                line(color, line_radius, [0.0, size as f64, size as f64, size as f64], transform, graphics);
            }
            Option::None => {}
        }
    }

    pub fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(button_args) => match button_args.state {
                ButtonState::Press => match button_args.button {
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
        if self.is_game_over {
            self.should_restart_level = true;
            return;
        }
        if self.direction == opposite_direction {
            return;
        }
        self.direction = new_direction
    }
}