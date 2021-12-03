use crate::verbs::move_sub;
use crate::Result;
use crate::{AppParams, Challenge};

pub fn solve_day_2_part_1(challenge: Challenge, params: AppParams) -> Result<()> {
    let challenge_data = if params.use_example_data {
        challenge.example_data
    } else {
        challenge.data
    };
    move_sub::run(challenge_data.lines(), move_sub::MovementStyle::LINEAR)
}

pub fn solve_day_2_part_2(challenge: Challenge, params: AppParams) -> Result<()> {
    let challenge_data = if params.use_example_data {
        challenge.example_data
    } else {
        challenge.data
    };
    move_sub::run(challenge_data.lines(), move_sub::MovementStyle::DIRECTIONAL)
}
