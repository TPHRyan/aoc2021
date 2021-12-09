mod challenge;
mod prompt;
mod solution;

pub use challenge::{get_challenge, Challenge};
pub use prompt::{prompt_select_challenge_day, prompt_select_challenge_part};
pub use solution::{get_challenge_solution, ChallengeSolution};
