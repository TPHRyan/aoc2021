use crate::sub::modelling::{simulate_lanternfish, Lanternfish};
use crate::{Error, Result};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    match challenge_data.lines().next() {
        Some(line) => {
            let initial_fish = parse_fish(line)?;
            let fish_count = simulate_lanternfish(initial_fish, 80);
            println!("{}", fish_count);
            Ok(())
        }
        None => Err(Box::new(Error::new(
            "No data found for lanternfish simulation!",
        ))),
    }
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}

fn parse_fish(fish_str: &str) -> Result<Vec<Lanternfish>> {
    let fish_ints_result: Vec<u32> = fish_str
        .split(",")
        .map(|n_str| u32::from_str(n_str))
        .collect::<std::result::Result<Vec<u32>, ParseIntError>>()?;
    Ok(fish_ints_result
        .iter()
        .map(|n| Lanternfish::from(*n))
        .collect::<Vec<Lanternfish>>())
}
