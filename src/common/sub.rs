mod movements;

use micromath::vector;

pub use movements::move_directional;
pub use movements::move_linear;

pub struct Submarine {
    pub position: Position,
    pub aim: i32,
    move_fn: SubMoveFn,
}

pub type SubMoveFn = fn(sub: &Submarine, direction: Direction, distance: i32) -> Submarine;

impl Submarine {
    pub fn new(move_fn: SubMoveFn) -> Submarine {
        Submarine {
            position: position(0, 0),
            aim: 0,
            move_fn,
        }
    }

    pub fn apply_move(&self, direction: Direction, distance: i32) -> Submarine {
        (self.move_fn)(self, direction, distance)
    }
}

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
