use crate::sub::heightmaps::{get_risk_levels, Heightmap};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let heightmap = parse_heightmap(challenge_data)?;
    let risk_levels = get_risk_levels(&heightmap);
    let risk_sum: u32 = risk_levels.values().fold(0, |acc, &risk| acc + risk as u32);
    println!("Sum of risk levels: {}", risk_sum);
    Ok(())
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}

fn parse_heightmap(challenge_data: String) -> Result<Heightmap> {
    Heightmap::from_str(&challenge_data)
}
