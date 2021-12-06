mod day_1;
mod day_2;
mod day_3;

use super::challenge::Challenge;
use crate::{AppParams, Result};

type SolutionFn = fn(String) -> Result<()>;
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
        (self.solution_fn)(resolve_challenge_data(challenge, params))
    }
}

fn resolve_challenge_data(challenge: Challenge, params: AppParams) -> String {
    if params.use_example_data || challenge.data == "" {
        challenge.example_data
    } else {
        challenge.data
    }
}

pub fn get_challenge_solution(day: u8, part: u8) -> Option<ChallengeSolution> {
    let maybe_fn_ptr: Option<SolutionFn> = match day {
        1 => match part {
            1 => Some(day_1::solve_part_1),
            2 => Some(day_1::solve_part_2),
            _ => None,
        },
        2 => match part {
            1 => Some(day_2::solve_part_1),
            2 => Some(day_2::solve_part_2),
            _ => None,
        },
        3 => match part {
            1 => Some(day_3::solve_part_1),
            _ => None,
        },
        _ => None,
    };
    create_solution_from(day, part, maybe_fn_ptr)
}

fn create_solution_from(
    day: u8,
    part: u8,
    maybe_fn_ptr: Option<SolutionFn>,
) -> Option<ChallengeSolution> {
    maybe_fn_ptr.map(|fn_ptr| ChallengeSolution::new(day, part, fn_ptr))
}
