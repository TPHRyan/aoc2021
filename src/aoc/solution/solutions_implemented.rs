include!(concat!(env!("OUT_DIR"), "/solution_modules.in"));

use super::{ChallengeSolution, SolutionFn};

static GET_SOLUTION_FN: fn(u8, u8) -> Option<SolutionFn> =
    include!(concat!(env!("OUT_DIR"), "/solutions_implemented.in"));

pub fn get_challenge_solution(day: u8, part: u8) -> Option<ChallengeSolution> {
    create_solution_from((GET_SOLUTION_FN)(day, part))
}

fn create_solution_from(maybe_fn_ptr: Option<SolutionFn>) -> Option<ChallengeSolution> {
    maybe_fn_ptr.map(|fn_ptr| ChallengeSolution::new(fn_ptr))
}
