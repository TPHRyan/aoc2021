mod day_1;
mod day_2;

use super::challenge::Challenge;
use crate::{AppParams, Result};

type SolutionFn = fn(Challenge, AppParams) -> Result<()>;
#[allow(dead_code)]
pub struct ChallengeSolution {
    day: u8,
    part: u8,
    solution_fn: SolutionFn,
}

impl ChallengeSolution {
    fn new(day: u8, part: u8, solution_fn: SolutionFn) -> ChallengeSolution {
        ChallengeSolution {
            day,
            part,
            solution_fn,
        }
    }

    pub fn run(&self, challenge: Challenge, params: AppParams) -> Result<()> {
        (self.solution_fn)(challenge, params)
    }
}

pub fn get_challenge_solution(day: u8, part: u8) -> Option<ChallengeSolution> {
    let solution_with_fn = |fn_ptr: SolutionFn| Some(ChallengeSolution::new(day, part, fn_ptr));
    match day {
        1 => match part {
            1 => solution_with_fn(day_1::solve_day_1_part_1),
            2 => solution_with_fn(day_1::solve_day_1_part_2),
            _ => None,
        },
        2 => match part {
            1 => solution_with_fn(day_2::solve_day_2_part_1),
            2 => solution_with_fn(day_2::solve_day_2_part_2),
            _ => None,
        },
        _ => None,
    }
}
