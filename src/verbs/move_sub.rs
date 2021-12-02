use std::str::FromStr;

use crate::common::{self, sub};
use crate::AppParams;

pub fn run(params: AppParams) -> Result<(), String> {
    let initial_position = sub::Position::new(0, 0);
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
            println!("Final position is {}", final_position);
            let position_product = final_position.h_pos * final_position.depth;
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

fn parse_instruction(instruction: String) -> Result<(sub::Direction, usize), String> {
    let words: Vec<&str> = instruction.split_whitespace().collect();
    if words.len() < 2 {
        return Err(format!("Not enough words in instruction: {}", instruction));
    }
    let direction = sub::Direction::from_str(words[0]);
    if let None = direction {
        return Err(format!("Unknown direction: {}", words[0]));
    }
    let magnitude = usize::from_str(words[1]);
    if let Err(_) = magnitude {
        return Err(format!("Invalid magnitude \"{}\" provided!", words[1]));
    }
    return Ok((direction.unwrap(), magnitude.unwrap()));
}

fn move_sub(direction: sub::Direction, magnitude: usize, sub_pos: sub::Position) -> sub::Position {
    match direction {
        sub::Direction::FORWARD => sub::Position::new(sub_pos.h_pos + magnitude, sub_pos.depth),
        sub::Direction::DOWN => sub::Position::new(sub_pos.h_pos, sub_pos.depth + magnitude),
        sub::Direction::UP => sub::Position::new(sub_pos.h_pos, sub_pos.depth - magnitude),
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
            .try_fold(sub::Position::new(0, 0), |prev, instruction| {
                parse_and_execute_instruction(String::from(instruction), prev)
            });
        assert!(position_result.is_ok());
        let position = position_result.unwrap();
        assert_eq!(15, position.h_pos);
        assert_eq!(10, position.depth);
    }
}
