
#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP
        }
    }
}
