use std::collections::VecDeque;

use rand::Rng;

use crate::draw_utils::Color;
use crate::food::Food;
use crate::map::Map;
use crate::maps::{create_walls_and_snake_structs1, create_walls_and_snake_structs2, create_walls_and_snake_structs3, create_walls_and_snake_structs4, create_walls_and_snake_structs5, verify_correct_snake_structure, verify_correct_walls_structure};
use crate::point::Point;
use crate::snake::Snake;

type Colors = (Color, Color);

// background and stroke colors of snake
pub fn get_snake_colors() -> Vec<Colors> {
    return vec![
        ([0.0, 0.0, 0.0, 1.0], [0.9, 0.0, 0.0, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [0.0, 0.9, 0.0, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.9, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [0.0, 1.0, 1.0, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 0.0, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [1.0, 0.0, 1.0, 1.0]),
        ([0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0]),
        ([0.0, 0.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]),
        ([0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0]),
        ([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0]),
    ];
}

// background and stroke colors of wall
pub fn get_wall_colors() -> Vec<Colors> {
    return vec![
        ([0.67, 0.65, 0.6, 1.0], [0.5, 0.5, 0.4, 1.0]),
        ([0.5, 0.44, 0.63, 1.0], [0.54, 0.49, 0.71, 1.0]),
        ([0.95, 0.4, 0.18, 1.0], [0.87, 0.85, 0.78, 1.0]),
        ([0.38, 0.35, 0.3, 1.0], [0.0, 0.0, 0.0, 1.0]),
    ];
}

pub fn create_stuff() -> (Snake, Map, Food) {
    let (walls, snake_coords) = take_random_walls_and_snake();
    let (wall_background_color, wall_stroke_color) = take_random_colors(get_wall_colors());
    verify_correct_walls_structure(&walls);
    verify_correct_snake_structure(&snake_coords);

    let map = Map::new(walls, wall_background_color, wall_stroke_color);

    let (snake_body_color, snake_stroke_color) = take_random_colors(get_snake_colors());
    let snake = Snake::new(snake_coords, snake_body_color, snake_stroke_color);

    let food = generate_food(&snake.coords, &map.coords);
    return (snake, map, food);
}

pub fn take_random_colors(colors: Vec<Colors>) -> Colors {
    let mut rng = rand::thread_rng();
    let color_idx = rng.gen_range(0..colors.len());

    return colors[color_idx];
}

fn take_random_walls_and_snake() -> (Vec<(i32, i32)>, VecDeque<Point>) {
    let mut rng = rand::thread_rng();
    let factories: Vec<fn() -> (Vec<(i32, i32)>, VecDeque<Point>)> = vec![
        create_walls_and_snake_structs1,
        create_walls_and_snake_structs2,
        create_walls_and_snake_structs3,
        create_walls_and_snake_structs4,
        create_walls_and_snake_structs5,
    ];
    let number = rng.gen_range(0..factories.len());

    return factories[number]();
}

pub fn generate_food(snake_coords: &VecDeque<Point>, wall_coords: &Vec<Point>) -> Food {
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
