use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub struct Position {
    pub h_pos: usize,
    pub depth: usize,
}

impl Position {
    pub fn new(h_pos: usize, depth: usize) -> Position {
        Position { h_pos, depth }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(h: {}, d: {})", self.h_pos, self.depth)
    }
}
