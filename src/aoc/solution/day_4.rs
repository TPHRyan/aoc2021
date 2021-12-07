use crate::sub::bingo;
use crate::{Error, Result};
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let mut segments = challenge_data.split("\n\n");
    let random_numbers = parse_random_numbers(segments.next())?;
    let boards = parse_board_inputs(segments)?;
    bingo::find_winning_bingo_boards(random_numbers, boards)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let mut segments = challenge_data.split("\n\n");
    let random_numbers = parse_random_numbers(segments.next())?;
    let boards = parse_board_inputs(segments)?;
    bingo::find_losing_bingo_boards(random_numbers, boards)
}

fn parse_random_numbers(random_number_str: Option<&str>) -> Result<Vec<u32>> {
    let random_number_result: std::result::Result<Vec<u32>, Box<Error>> = match random_number_str {
        Some(s) => s
            .split(",")
            .map(|n_input| {
                u32::from_str(n_input).map_err(|err| Box::new(Error::new(&err.to_string())))
            })
            .collect(),
        None => Err(Box::new(Error::new("No data found for bingo game!"))),
    };
    Ok(random_number_result?)
}

fn parse_board_inputs<'a, T>(board_inputs: T) -> Result<Vec<bingo::BingoBoard>>
where
    T: Iterator<Item = &'a str>,
{
    board_inputs
        .map(|board_input| bingo::BingoBoard::from_str(board_input))
        .collect()
}
