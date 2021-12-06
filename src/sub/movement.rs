mod styles;

use std::str::FromStr;

use super::Submarine;
use crate::{Error, Result};
pub use styles::MovementStyle;

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

pub type SubMoveFn = fn(sub: &Submarine, direction: Direction, distance: i32) -> Submarine;

pub fn run<'a, T>(move_instructions: T, movement_style: MovementStyle) -> Result<()>
where
    T: Iterator<Item = &'a str>,
{
    let init_sub = Submarine::new(styles::get_movement_fn(movement_style));
    let final_result: Result<Submarine> =
        parse_and_execute_instructions(init_sub, move_instructions);
    match final_result {
        Ok(final_sub) => {
            let final_position = final_sub.position;
            println!(
                "Final position is (h: {}, d: {})",
                final_position.x, final_position.y
            );
            let position_product = final_position.x * final_position.y;
            println!("Final position product is {}", position_product);
            Ok(())
        }
        Err(message) => Err(message),
    }
}

fn parse_and_execute_instructions<'a, T>(
    sub: Submarine,
    mut move_instructions: T,
) -> Result<Submarine>
where
    T: Iterator<Item = &'a str>,
{
    move_instructions.try_fold(sub, |prev_sub, instruction| {
        parse_and_execute_instruction(instruction, prev_sub)
    })
}

fn parse_and_execute_instruction(instruction: &str, sub: Submarine) -> Result<Submarine> {
    if instruction == "" {
        return Ok(sub);
    }
    let (direction, distance) = parse_instruction(instruction)?;
    Ok(sub.apply_move(direction, distance))
}

fn parse_instruction(instruction: &str) -> Result<(super::Direction, i32)> {
    let words: Vec<&str> = instruction.split_whitespace().collect();
    if words.len() < 2 {
        return Err(Box::new(Error::new(&format!(
            "Not enough words in instruction: {}",
            instruction
        ))));
    }
    let direction = super::Direction::from_str(words[0]);
    if let None = direction {
        return Err(Box::new(Error::new(&format!(
            "Unknown direction: {}",
            words[0]
        ))));
    }
    let magnitude = i32::from_str(words[1]);
    if let Err(_) = magnitude {
        return Err(Box::new(Error::new(&format!(
            "Invalid magnitude \"{}\" provided!",
            words[1]
        ))));
    }
    return Ok((direction.unwrap(), magnitude.unwrap()));
}

#[cfg(test)]
mod tests {
    use super::{parse_and_execute_instructions, styles, Result, Submarine};

    const TEST_STR: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn move_directional_works_on_test_data() {
        let final_result: Result<Submarine> = parse_and_execute_instructions(
            Submarine::new(styles::move_directional),
            TEST_STR.lines(),
        );
        assert!(final_result.is_ok());
        let final_sub = final_result.unwrap();
        let position = final_sub.position;
        assert_eq!(15, position.x);
        assert_eq!(60, position.y);
    }

    #[test]
    fn move_linear_works_on_test_data() {
        let final_result: Result<Submarine> =
            parse_and_execute_instructions(Submarine::new(styles::move_linear), TEST_STR.lines());
        assert!(final_result.is_ok());
        let final_sub = final_result.unwrap();
        let position = final_sub.position;
        assert_eq!(15, position.x);
        assert_eq!(10, position.y);
    }
}
