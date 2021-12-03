use std::str::FromStr;

use crate::common;
use crate::common::sub::{self, Submarine};
use crate::AppParams;

pub fn run(params: AppParams) -> Result<(), String> {
    let initial_sub: Submarine = Submarine::new(sub::move_linear);
    let mut move_instructions = common::read_data_lines(params);
    let position_result: Result<Submarine, String> =
        move_instructions.try_fold(initial_sub, |prev_sub, instruction_result| {
            match instruction_result {
                Ok(instruction) => parse_and_execute_instruction(instruction, prev_sub),
                Err(err) => Err(format!("{}", err)),
            }
        });
    match position_result {
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

fn parse_and_execute_instruction(instruction: String, sub: Submarine) -> Result<Submarine, String> {
    if instruction == "" {
        return Ok(sub);
    }
    let (direction, distance) = parse_instruction(instruction)?;
    Ok(sub.apply_move(direction, distance))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_linear_works_on_test_data() {
        let test_str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let mut test_instructions = test_str.lines();
        let final_result: Result<Submarine, String> = test_instructions.try_fold(
            Submarine::new(sub::move_linear),
            |prev_sub, instruction| {
                parse_and_execute_instruction(String::from(instruction), prev_sub)
            },
        );
        assert!(final_result.is_ok());
        let final_sub = final_result.unwrap();
        let position = final_sub.position;
        assert_eq!(15, position.x);
        assert_eq!(10, position.y);
    }
}
