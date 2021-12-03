use super::{position, Direction, Submarine};

pub fn move_directional(sub: &Submarine, direction: Direction, distance: i32) -> Submarine {
    Submarine {
        position: match direction {
            Direction::FORWARD => sub.position + position(distance, distance * sub.aim),
            _ => sub.position,
        },
        aim: match direction {
            Direction::DOWN => sub.aim + distance,
            Direction::UP => sub.aim - distance,
            _ => sub.aim,
        },
        ..*sub
    }
}

#[allow(dead_code)]
pub fn move_linear(sub: &Submarine, direction: Direction, distance: i32) -> Submarine {
    Submarine {
        position: match direction {
            Direction::FORWARD => sub.position + position(distance, 0),
            Direction::DOWN => sub.position + position(0, distance),
            Direction::UP => sub.position + position(0, -distance),
        },
        ..*sub
    }
}
