use crate::verbs::move_sub;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    move_sub::run(challenge_data.lines(), move_sub::MovementStyle::LINEAR)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    move_sub::run(challenge_data.lines(), move_sub::MovementStyle::DIRECTIONAL)
}
