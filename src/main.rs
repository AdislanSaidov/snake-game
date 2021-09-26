extern crate piston_window;

use piston_window::*;
use std::collections::VecDeque;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    NONE
}

struct Point {
    x: f64,
    y: f64
}

struct Game {
    xDelta: f64,
    yDelta: f64,
    x: f64,
    y: f64,
    direction: Direction,
    size: f64,
    cell_size: f64,
    cell_count: i32,
    snake_coords: VecDeque<Point>
}

impl Game {
    fn new() -> Game {
        let mut deque: VecDeque<Point> = VecDeque::new();
        deque.push_back(Point { x: 2.0, y: 2.0 });
        deque.push_back(Point { x: 2.0, y: 3.0 });
        deque.push_back(Point { x: 2.0, y: 4.0 });

        Game {
            xDelta: 1.0,
            yDelta: 1.0,
            x: 1.0,
            y: 1.0,
            direction: Direction::NONE,
            size: 20.0,
            cell_size: 20.0,
            cell_count: 30,
            snake_coords: deque
        }
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
        let v = (5.0 * upd.dt);
        match self.direction {
            Direction::LEFT => {
                self.xDelta -= v;
            }
            Direction::RIGHT => {
                self.xDelta += v;
            }
            Direction::TOP => {
                self.yDelta -= v;
            }
            Direction::BOTTOM => {
                self.yDelta += v;
            }
            Direction::NONE => {

            }
        }
        if self.direction != Direction::NONE {
            let oldX = self.x;
            let oldY = self.y;
            self.x = self.xDelta.round();
            self.y = self.yDelta.round();
            if oldX == self.x && oldY == self.y{
                return;
            }
            self.snake_coords.push_front(Point{ x: self.x, y: self.y });
            self.snake_coords.pop_back();
            println!("{}", upd.dt);
            println!("{}", self.snake_coords.len().to_string());
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
                println!("LOOP {} {}", point.x, point.y);
                let square = rectangle::square(0.0, 0.0, self.size);
                let center = c.transform.trans(point.x * self.cell_size as f64, point.y * self.cell_size as f64);
                rectangle(
                    red,
                    square,
                    center,
                    g,
                );
            }

        });
    }

    fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(but) => match but.state {
                ButtonState::Press => match but.button {
                    Button::Keyboard(Key::Up) => {
                        self.direction = Direction::TOP
                    },
                    Button::Keyboard(Key::Down) => {
                        self.direction = Direction::BOTTOM
                    },
                    Button::Keyboard(Key::Left) => {
                        self.direction = Direction::LEFT
                    },
                    Button::Keyboard(Key::Right) => {
                        self.direction = Direction::RIGHT
                    },
                    _ => (),
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("piston-tutorial", [600, 600])
        .exit_on_esc(true)
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