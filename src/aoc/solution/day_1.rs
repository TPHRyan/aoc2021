use crate::common;
use crate::verbs::depth_scan;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    depth_scan::run(common::int_lines(&challenge_data), 1)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    depth_scan::run(common::int_lines(&challenge_data), 3)
}
