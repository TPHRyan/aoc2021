use crate::common;
use crate::sub::scanning;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    scanning::run_depth_scan(common::int_lines(&challenge_data), 1)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    scanning::run_depth_scan(common::int_lines(&challenge_data), 3)
}
