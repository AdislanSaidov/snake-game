use graphics::types::ColorComponent;
use crate::snake::Snake;
use crate::direction::Direction;
use std::collections::VecDeque;
use crate::point::Point;
use crate::map::Map;
use crate::food::Food;
use rand::Rng;

fn snake_body_colors() -> Vec<[ColorComponent; 4]> {
    return vec![
        [0.0, 0.0, 0.0, 1.0]
    ];
}

fn snake_stroke_colors() -> Vec<[ColorComponent; 4]> {
    return vec![
        [0.9, 0.0, 0.0, 1.0]
    ];
}

fn wall_background_colors() -> Vec<[ColorComponent; 4]> {
    return vec![
        [0.67, 0.65, 0.6, 1.0]
    ];
}

fn wall_stroke_colors() -> Vec<[ColorComponent; 4]> {
    return vec![
        [0.5, 0.5, 0.4, 1.0]
    ];
}

pub fn create_stuff() -> (Snake, Map, Food) {

    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((15, 15));
    deque.push_back((15, 16));
    deque.push_back((15, 17));

    let walls = vec![
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
    ];
    let wall_background_color = take_random_color(wall_background_colors());
    let wall_stroke_color = take_random_color(wall_stroke_colors());
    let map = Map::new(walls, wall_background_color, wall_stroke_color);

    let snake_body_color = take_random_color(snake_body_colors());
    let snake_stroke_color = take_random_color(snake_stroke_colors());
    let snake = Snake::new(15., 15., Direction::TOP, deque, snake_body_color, snake_stroke_color);

    let food = generate_food(&snake.coords, &map.coords);
    return (snake, map, food);
}

fn take_random_color(colors: Vec<[ColorComponent; 4]>) -> [ColorComponent; 4] {
    let mut rng = rand::thread_rng();
    let color_idx = rng.gen_range(0..colors.len());

    return colors[color_idx]
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
