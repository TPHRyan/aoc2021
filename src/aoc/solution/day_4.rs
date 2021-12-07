use crate::sub::bingo;
use crate::{Error, Result};
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let mut segments = challenge_data.split("\n\n");

    let random_numbers_result: std::result::Result<Vec<u32>, Box<Error>> = match segments.next() {
        Some(s) => s
            .split(",")
            .map(|n_input| {
                u32::from_str(n_input).map_err(|err| Box::new(Error::new(&err.to_string())))
            })
            .collect(),
        None => Err(Box::new(Error::new("No data found for bingo game!"))),
    };
    let mut board_inputs: Vec<String> = Vec::new();
    while let Some(board_input) = segments.next() {
        board_inputs.push(String::from(board_input));
    }
    bingo::run_bingo(random_numbers_result?, board_inputs)
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}
