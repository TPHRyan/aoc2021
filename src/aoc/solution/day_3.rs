use crate::common::bit_lines;
use crate::sub::diagnostics;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let binary_numbers: Vec<Vec<u8>> = bit_lines(&challenge_data);
    diagnostics::run_gamma_epsilon_report(binary_numbers)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let binary_numbers: Vec<Vec<u8>> = bit_lines(&challenge_data);
    diagnostics::run_life_support_rating_report(binary_numbers)
}
