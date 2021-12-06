use crate::common::bit_lines;
use crate::verbs::process_diagnostics;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let binary_numbers: Vec<Vec<u8>> = bit_lines(&challenge_data);
    process_diagnostics::run(binary_numbers)
}
