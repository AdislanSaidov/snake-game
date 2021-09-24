extern crate piston_window;

use piston_window::*;
use std::collections::VecDeque;

struct Point {
    x: i32,
    y: i32
}

struct Game {
    x: f64,
    y: f64,
    up_d: bool,
    down_d: bool,
    left_d: bool,
    right_d: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            x: 0.0,
            y: 0.0,
            up_d: false,
            down_d: false,
            left_d: false,
            right_d: false
        }
    }

    fn on_update(&mut self, upd: &UpdateArgs) {


    }

    fn on_draw(&mut self, ren: &RenderArgs, window: &mut PistonWindow, event: &Event) {
        let window_dimensions = ren.draw_size;
        let width = window_dimensions[0];
        let height = window_dimensions[1];
        window.draw_2d(event, |c, g, _device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

        });
    }

    fn on_input(&mut self, inp: &Input) {
        match inp {
            Input::Button(but) => match but.state {
                ButtonState::Press => match but.button {
                    Button::Keyboard(Key::Up) => {
                        self.down_d = false;
                        self.left_d = false;
                        self.right_d = false;
                        self.up_d = true;
                    },
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                        self.left_d = false;
                        self.right_d = false;
                        self.up_d = false;
                    },
                    Button::Keyboard(Key::Left) => {
                        self.down_d = false;
                        self.left_d = true;
                        self.right_d = false;
                        self.up_d = false;
                    },
                    Button::Keyboard(Key::Right) => {
                        self.down_d = false;
                        self.left_d = false;
                        self.right_d = true;
                        self.up_d = false;
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