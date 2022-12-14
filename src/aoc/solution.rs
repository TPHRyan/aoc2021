mod solutions_implemented;

use super::challenge::Challenge;
use crate::{AppParams, Result};
pub use solutions_implemented::get_challenge_solution;

type SolutionFn = fn(String) -> Result<()>;
pub struct ChallengeSolution {
    solution_fn: SolutionFn,
}

impl ChallengeSolution {
    fn new(solution_fn: SolutionFn) -> ChallengeSolution {
        ChallengeSolution { solution_fn }
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
