use crate::snake::Snake;
use std::collections::VecDeque;
use crate::point::Point;
use crate::direction::Direction;
use crate::resources::{take_random_colors, snake_colors};

pub fn map1() -> Vec<(i32, i32)>{
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
    ];
}

pub fn map_and_snake1() -> (Vec<(i32, i32)>, Snake){
    let walls = map1();
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((15, 15));
    deque.push_back((15, 16));
    deque.push_back((15, 17));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(15., 15., Direction::TOP, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

pub fn map_and_snake2() -> (Vec<(i32, i32)>, Snake){
    let walls = vec![];
    let mut deque: VecDeque<Point> = VecDeque::new();
    deque.push_back((15, 15));
    deque.push_back((15, 16));
    deque.push_back((15, 17));

    let (snake_body_color, snake_stroke_color) = take_random_colors(snake_colors());
    let snake = Snake::new(15., 15., Direction::TOP, deque, snake_body_color, snake_stroke_color);

    return (walls, snake);
}

