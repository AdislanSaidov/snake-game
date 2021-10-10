mod food;
mod point;
mod direction;
mod game;

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
use crate::game::Game;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 600.0;


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