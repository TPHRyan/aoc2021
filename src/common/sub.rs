use micromath::vector;

#[derive(Debug)]
pub enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Direction {
    pub fn from_str(str: &str) -> Option<Direction> {
        match str {
            "forward" => Some(Direction::FORWARD),
            "down" => Some(Direction::DOWN),
            "up" => Some(Direction::UP),
            _ => None,
        }
    }
}

pub type Position = vector::Vector2d<i32>;

pub fn position(x: i32, y: i32) -> Position {
    vector::Vector2d { x, y }
}
