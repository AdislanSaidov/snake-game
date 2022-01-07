extern crate find_folder;
extern crate piston_window;

use piston_window::*;

use crate::game::Game;
use crate::resources::create_stuff;

mod food;
mod point;
mod direction;
mod game;
mod snake;
mod map;
mod draw_utils;
mod bottom_bar;
mod state;
mod game_button;
mod resources;
mod snake_config;
mod maps;

static FONT_NAME: &str = "Hunger Games.ttf";

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 640.0;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake game", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Unable to build a window");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Unable to read assets");

    let glyphs = window.load_font(assets.join(FONT_NAME))
        .expect("Unable to load font");

    let (snake, map, food) = create_stuff();
    let mut game = Game::new(glyphs, snake, map, food);

    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            game.on_mouse_input(pos)
        }
        match event {
            Event::Loop(Loop::Update(ref upd)) => game.on_update(upd),
            Event::Loop(Loop::Render(_)) => game.on_draw(&mut window, &event),
            Event::Input(ref inp, _) => game.on_input(inp),
            _ => {}
        }
    }
}
