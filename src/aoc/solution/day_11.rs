use crate::sub::modelling::Octopuses;
use crate::Result;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let mut octopuses = Octopuses::from_str(&challenge_data)?;
    let num_flashes = octopuses.simulate(100);
    println!("{} flashes occurred after 100 steps.", num_flashes);
    Ok(())
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}
