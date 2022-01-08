use std::collections::VecDeque;

use crate::point::Point;
use crate::utils::has_duplicates;

// without walls
fn create_map1() -> Vec<(i32, i32)> {
    return vec![];
}

fn create_map2() -> Vec<(i32, i32)> {
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

fn create_map3() -> Vec<(i32, i32)> {
    return vec![
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
        (0, 10), (0, 11), (0, 12), (0, 17), (0, 18), (0, 19),
        (0, 20), (0, 21), (0, 22), (0, 23), (0, 24), (0, 25), (0, 26), (0, 27), (0, 28), (0, 29),

        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0),
        (11, 0), (12, 0), (17, 0), (18, 0), (19, 0), (20, 0),
        (21, 0), (22, 0), (23, 0), (24, 0), (25, 0), (26, 0), (27, 0), (28, 0), (29, 0),

        (1, 29), (2, 29), (3, 29), (4, 29), (5, 29), (6, 29), (7, 29), (8, 29), (9, 29), (10, 29),
        (11, 29), (12, 29), (17, 29), (18, 29), (19, 29), (20, 29),
        (21, 29), (22, 29), (23, 29), (24, 29), (25, 29), (26, 29), (27, 29), (28, 29), (29, 29),

        (29, 1), (29, 2), (29, 3), (29, 4), (29, 5), (29, 6), (29, 7), (29, 8), (29, 9),
        (29, 10), (29, 11), (29, 12), (29, 17), (29, 18), (29, 19),
        (29, 20), (29, 21), (29, 22), (29, 23), (29, 24), (29, 25), (29, 26), (29, 27), (29, 28),

        (13, 13), (13, 14), (13, 15), (13, 16),
        (14, 13), (14, 14), (14, 15), (14, 16),
        (15, 13), (15, 14), (15, 15), (15, 16),
        (16, 13), (16, 14), (16, 15), (16, 16)
    ];
}

fn create_map4() -> Vec<(i32, i32)> {
    return vec![
        (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13),
        (13, 0), (13, 1), (13, 2), (13, 3), (13, 4), (13, 5), (13, 6), (13, 7), (13, 8), (13, 9), (13, 10), (13, 11), (13, 12),

        (17, 17), (17, 18), (17, 19), (17, 20), (17, 21), (17, 22), (17, 23), (17, 24), (17, 25), (17, 26), (17, 27), (17, 28), (17, 29),
        (18, 17), (19, 17), (20, 17), (21, 17), (22, 17), (23, 17), (24, 17), (25, 17), (26, 17), (27, 17), (28, 17), (29, 17)
    ];
}

fn create_map5() -> Vec<(i32, i32)> {
    return vec![
        (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1),
        (18, 1), (19, 1), (20, 1), (21, 1), (22, 1), (23, 1), (24, 1), (25, 1), (26, 1), (27, 1), (28, 1),

        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8),

        (1, 28), (2, 28), (3, 28), (4, 28), (5, 28), (6, 28), (7, 28), (8, 28), (9, 28), (10, 28), (11, 28), (12, 28),
        (18, 28), (19, 28), (20, 28), (21, 28), (22, 28), (23, 28), (24, 28), (25, 28), (26, 28), (27, 28), (28, 28),

        (1, 9), (1, 10), (1, 11),
        (1, 27), (1, 26), (1, 25), (1, 24), (1, 23), (1, 22), (1, 21), (1, 20), (1, 19), (1, 18),

        (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11),
        (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18),

        (7, 12), (7, 13), (7, 14), (7, 15), (7, 16), (7, 17),

        (12, 2), (12, 3), (12, 4), (12, 5), (12, 6),
        (18, 2), (18, 3), (18, 4), (18, 5), (18, 6),

        (13, 6), (14, 6), (15, 6), (16, 6), (17, 6),

        (12, 27), (12, 26), (12, 25), (12, 24), (12, 23),
        (18, 27), (18, 26), (18, 25), (18, 24), (18, 23),

        (28, 18), (28, 19), (28, 20), (28, 21), (28, 22), (28, 23), (28, 24), (28, 25), (28, 26), (28, 27),
        (23, 18), (24, 18), (25, 18), (26, 18), (27, 18),

        (28, 2), (28, 3), (28, 4), (28, 5), (28, 6), (28, 7), (28, 8), (28, 9), (28, 10), (28, 11),
        (23, 11), (24, 11), (25, 11), (26, 11), (27, 11)
    ];
}

pub fn create_walls_and_snake_structs1() -> (Vec<Point>, VecDeque<Point>) {
    let walls = create_map1();
    let mut snake_coords: VecDeque<Point> = VecDeque::new();
    snake_coords.push_back((15, 17));
    snake_coords.push_back((16, 17));
    snake_coords.push_back((17, 17));

    return (walls, snake_coords);
}

pub fn create_walls_and_snake_structs2() -> (Vec<Point>, VecDeque<Point>) {
    let walls = create_map2();
    let mut snake_coords: VecDeque<Point> = VecDeque::new();
    snake_coords.push_back((15, 15));
    snake_coords.push_back((15, 16));
    snake_coords.push_back((15, 17));

    return (walls, snake_coords);
}

pub fn create_walls_and_snake_structs3() -> (Vec<Point>, VecDeque<Point>) {
    let walls = create_map3();
    let mut snake_coords: VecDeque<Point> = VecDeque::new();
    snake_coords.push_back((2, 15));
    snake_coords.push_back((1, 15));
    snake_coords.push_back((0, 15));

    return (walls, snake_coords);
}

pub fn create_walls_and_snake_structs4() -> (Vec<Point>, VecDeque<Point>) {
    let walls = create_map4();
    let mut snake_coords: VecDeque<Point> = VecDeque::new();
    snake_coords.push_back((2, 15));
    snake_coords.push_back((1, 15));
    snake_coords.push_back((0, 15));

    return (walls, snake_coords);
}

pub fn create_walls_and_snake_structs5() -> (Vec<Point>, VecDeque<Point>) {
    let walls = create_map5();
    let mut snake_coords: VecDeque<Point> = VecDeque::new();
    snake_coords.push_back((27, 15));
    snake_coords.push_back((28, 15));
    snake_coords.push_back((29, 15));

    return (walls, snake_coords);
}

pub fn verify_correct_snake_structure(snake_coords: &VecDeque<Point>) {
    if snake_coords.len() < MIN_SNAKE_LEN {
        panic!("Snake structure must contain at least {} cells", MIN_SNAKE_LEN);
    }
    if has_duplicates(snake_coords) {
        panic!("Snake structure contains duplicate cells");
    }
    for idx in 0..snake_coords.len() {
        let next_option = if idx < snake_coords.len() - 1 {
            snake_coords.get(idx + 1)
        } else {
            Option::None
        };

        let current_option = snake_coords.get(idx);

        match (current_option, next_option) {
            (Some(&current), Some(&next)) => {
                let x_diff = (current.0 - next.0).abs();
                let y_diff = (current.1 - next.1).abs();

                if x_diff > 1 || y_diff > 1 || x_diff == 1 && y_diff == 1 {
                    panic!("Incorrect snake structure: unconnected points found, no points between {:?} and {:?}", current, next);
                }
            }
            _ => {}
        }
    }

}

pub fn verify_correct_walls_structure(walls: &Vec<Point>) {
    if has_duplicates(walls) {
        panic!("Walls structure contains duplicate cells");
    }
}

const MIN_SNAKE_LEN: usize = 3;
