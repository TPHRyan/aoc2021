use crate::sub::movement as sub_movement;
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    sub_movement::run(challenge_data.lines(), sub_movement::MovementStyle::LINEAR)
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    sub_movement::run(
        challenge_data.lines(),
        sub_movement::MovementStyle::DIRECTIONAL,
    )
}
