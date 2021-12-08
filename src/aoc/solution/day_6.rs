use crate::sub::modelling::{simulate_lanternfish, Lanternfish};
use crate::{Error, Result};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let fish_line = get_first_data_line(&challenge_data)?;
    let initial_fish = parse_fish(&fish_line)?;
    let fish_count = simulate_lanternfish(initial_fish, 80);
    println!("{}", fish_count);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let fish_line = get_first_data_line(&challenge_data)?;
    let initial_fish = parse_fish(&fish_line)?;
    let fish_count = simulate_lanternfish(initial_fish, 256);
    println!("{}", fish_count);
    Ok(())
}

fn get_first_data_line(challenge_data: &str) -> Result<String> {
    match challenge_data.lines().next() {
        Some(line) => Ok(String::from(line)),
        None => Err(Box::new(Error::new(
            "No data found for lanternfish simulation!",
        ))),
    }
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
