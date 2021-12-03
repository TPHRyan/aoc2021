use crate::common;
use crate::verbs::depth_scan;
use crate::Result;
use crate::{AppParams, Challenge};

pub fn solve_day_1_part_1(challenge: Challenge, params: AppParams) -> Result<()> {
    let challenge_data = if params.use_example_data {
        challenge.example_data
    } else {
        challenge.data
    };
    depth_scan::run(common::int_lines(&challenge_data), 1)
}

pub fn solve_day_1_part_2(challenge: Challenge, params: AppParams) -> Result<()> {
    let challenge_data = if params.use_example_data {
        challenge.example_data
    } else {
        challenge.data
    };
    depth_scan::run(common::int_lines(&challenge_data), 3)
}
