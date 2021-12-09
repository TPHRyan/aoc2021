use crate::{prompt, Result};

pub fn prompt_select_challenge_day() -> Result<u8> {
    return prompt::prompt_select_bounded_number("Please select a challenge. (1-25)", 1, 25);
}

pub fn prompt_select_challenge_part(num_parts: u8) -> Result<u8> {
    return prompt::prompt_select_bounded_number(
        &format!("Which challenge part? (1-{})", num_parts),
        1,
        num_parts,
    );
}
