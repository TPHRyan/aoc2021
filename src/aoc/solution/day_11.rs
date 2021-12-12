use crate::sub::modelling::Octopuses;
use crate::Result;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let mut octopuses = Octopuses::from_str(&challenge_data)?;
    let num_flashes = octopuses.simulate(100);
    println!("{} flashes occurred after 100 steps.", num_flashes);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let mut octopuses = Octopuses::from_str(&challenge_data)?;
    let mut num_ticks = 0;
    let mut last_flashes = 0;
    while last_flashes < 100 {
        last_flashes = octopuses.tick();
        num_ticks += 1;
    }
    println!("{} ticks for all octopuses to flash", num_ticks);
    Ok(())
}
