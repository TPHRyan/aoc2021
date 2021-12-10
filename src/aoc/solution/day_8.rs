use crate::sub::crypto::seven_segment::{self, OutputDisplay};
use crate::sub::crypto::{filter_non_unique_digits, unscramble_outputs};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let outputs = parse_outputs_only(challenge_data)?;
    let unique_digits = filter_non_unique_digits(&outputs);
    println!("Number of unique digits: {}", unique_digits.len());
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let signals_and_outputs = parse_signals_and_outputs(challenge_data)?;
    let unscrambled_outputs = unscramble_outputs(&signals_and_outputs)?;
    let output_sum: u32 = unscrambled_outputs
        .iter()
        .fold(0, |acc, &output_number| acc + output_number);
    println!("Sum of outputs: {}", output_sum);
    Ok(())
}

fn parse_signals_and_outputs(challenge_data: String) -> Result<Vec<(Vec<u8>, OutputDisplay)>> {
    seven_segment::signal_output_lines_to_signals_and_output(&challenge_data)
}

fn parse_outputs_only(challenge_data: String) -> Result<Vec<OutputDisplay>> {
    let signals_and_outputs = parse_signals_and_outputs(challenge_data)?;
    Ok(signals_and_outputs
        .into_iter()
        .map(|(_, output)| output)
        .collect())
}
