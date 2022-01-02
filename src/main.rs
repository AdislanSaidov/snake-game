extern crate find_folder;
extern crate piston_window;

use piston_window::*;

use crate::game::Game;

mod food;
mod point;
mod direction;
mod game;

static FONT_NAME: &str = "Hunger Games.ttf";

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 640.0;
const BOTTOM_BAR_HEIGHT: f64 = 40.0;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake game", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join(FONT_NAME)).unwrap();

    let mut game = Game::new(glyphs);

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Update(ref upd)) => game.on_update(upd),
            Event::Loop(Loop::Render(ref ren)) => game.on_draw(ren, &mut window, &event),
            Event::Input(ref inp, _) => game.on_input(inp),
            _ => {}
        }
    }
}