use crate::sub::modelling::{calculate_cheapest_alignment_fuel, FuelModel};
use crate::{common, Result};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let subs = parse_crab_subs(challenge_data)?;
    let cheapest_fuel_cost = calculate_cheapest_alignment_fuel(subs, FuelModel::LINEAR);
    println!("Cheapest fuel cost: {}", cheapest_fuel_cost);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let subs = parse_crab_subs(challenge_data)?;
    let cheapest_fuel_cost = calculate_cheapest_alignment_fuel(subs, FuelModel::TRIANGULAR);
    println!("Cheapest fuel cost: {}", cheapest_fuel_cost);
    Ok(())
}

fn parse_crab_subs(challenge_data: String) -> Result<Vec<u32>> {
    let subs_line = common::first_line(&challenge_data)?;
    let subs: std::result::Result<Vec<u32>, ParseIntError> =
        subs_line.split(",").map(|s| u32::from_str(s)).collect();
    Ok(subs?)
}
