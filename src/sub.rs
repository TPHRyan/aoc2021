pub mod bingo;
pub mod diagnostics;
pub mod movement;
pub mod pathfinding;
pub mod scanning;

use micromath::vector;
pub use movement::{Direction, SubMoveFn};

pub struct Submarine {
    pub position: Position,
    pub aim: i32,
    move_fn: SubMoveFn,
}

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

pub type Position = vector::Vector2d<i32>;

pub fn position(x: i32, y: i32) -> Position {
    vector::Vector2d { x, y }
}
