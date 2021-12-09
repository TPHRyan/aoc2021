use crate::sub::crypto::{filter_non_unique_digits, OutputDisplay};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let outputs = parse_outputs_only(challenge_data)?;
    let unique_digits = filter_non_unique_digits(&outputs)?;
    println!("Number of unique digits: {}", unique_digits.len());
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    Ok(())
}

fn parse_outputs_only(challenge_data: String) -> Result<Vec<OutputDisplay>> {
    let outputs_str: Vec<&str> = challenge_data
        .lines()
        .map(|line| line.split(" | ").last())
        .filter_map(|output_str| output_str)
        .collect();
    outputs_str
        .iter()
        .map(|&output_str| OutputDisplay::from_str(output_str))
        .collect()
}
