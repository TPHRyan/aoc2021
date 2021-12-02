use std::str::FromStr;

use crate::common::{self, sub};
use crate::AppParams;

pub fn run(params: AppParams) -> Result<(), String> {
    let initial_position: sub::Position = sub::position(0, 0);
    let mut move_instructions = common::read_data_lines(params);
    let position_result: Result<sub::Position, String> =
        move_instructions.try_fold(initial_position, |prev, instruction_result| {
            match instruction_result {
                Ok(instruction) => parse_and_execute_instruction(instruction, prev),
                Err(err) => Err(format!("{}", err)),
            }
        });
    match position_result {
        Ok(final_position) => {
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

fn parse_and_execute_instruction(
    instruction: String,
    sub_pos: sub::Position,
) -> Result<sub::Position, String> {
    if instruction == "" {
        return Ok(sub_pos);
    }
    let (direction, magnitude) = parse_instruction(instruction)?;
    Ok(move_sub(direction, magnitude, sub_pos))
}

fn parse_instruction(instruction: String) -> Result<(sub::Direction, i32), String> {
    let words: Vec<&str> = instruction.split_whitespace().collect();
    if words.len() < 2 {
        return Err(format!("Not enough words in instruction: {}", instruction));
    }
    let direction = sub::Direction::from_str(words[0]);
    if let None = direction {
        return Err(format!("Unknown direction: {}", words[0]));
    }
    let magnitude = i32::from_str(words[1]);
    if let Err(_) = magnitude {
        return Err(format!("Invalid magnitude \"{}\" provided!", words[1]));
    }
    return Ok((direction.unwrap(), magnitude.unwrap()));
}

fn move_sub(direction: sub::Direction, magnitude: i32, sub_pos: sub::Position) -> sub::Position {
    match direction {
        sub::Direction::FORWARD => sub_pos + sub::position(magnitude, 0),
        sub::Direction::DOWN => sub_pos + sub::position(0, magnitude),
        sub::Direction::UP => sub_pos + sub::position(0, -magnitude),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_sub_works_on_test_data() {
        let test_str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let mut test_instructions = test_str.lines();
        let position_result: Result<sub::Position, String> = test_instructions
            .try_fold(sub::position(0, 0), |prev, instruction| {
                parse_and_execute_instruction(String::from(instruction), prev)
            });
        assert!(position_result.is_ok());
        let position = position_result.unwrap();
        assert_eq!(15, position.x);
        assert_eq!(10, position.y);
    }
}
