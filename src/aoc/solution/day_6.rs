use crate::sub::modelling::{simulate_lanternfish, Lanternfish};
use crate::{common, Error, Result};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let initial_fish = parse_fish(challenge_data)?;
    let fish_count = simulate_lanternfish(initial_fish, 80);
    println!("{}", fish_count);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let initial_fish = parse_fish(challenge_data)?;
    let fish_count = simulate_lanternfish(initial_fish, 256);
    println!("{}", fish_count);
    Ok(())
}

fn parse_fish(challenge_data: String) -> Result<Vec<Lanternfish>> {
    let fish_str = common::first_line(&challenge_data)?;
    let fish_ints_result: Vec<u32> = fish_str
        .split(",")
        .map(|n_str| u32::from_str(n_str))
        .collect::<std::result::Result<Vec<u32>, ParseIntError>>()?;
    Ok(fish_ints_result
        .iter()
        .map(|n| Lanternfish::from(*n))
        .collect::<Vec<Lanternfish>>())
}
