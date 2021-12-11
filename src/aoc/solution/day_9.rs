use crate::sub::heightmaps::{get_basin_sizes, get_risk_levels, Heightmap};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let heightmap = parse_heightmap(challenge_data)?;
    let risk_levels = get_risk_levels(&heightmap);
    let risk_sum: u32 = risk_levels.values().fold(0, |acc, &risk| acc + risk as u32);
    println!("Sum of risk levels: {}", risk_sum);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let heightmap = parse_heightmap(challenge_data)?;
    let mut basin_sizes: Vec<usize> = get_basin_sizes(&heightmap, true);
    basin_sizes.reverse();
    let basin_product = basin_sizes[..3].iter().fold(1, |acc, size| acc * size);
    println!("Product of basin sizes: {}", basin_product);
    Ok(())
}

fn parse_heightmap(challenge_data: String) -> Result<Heightmap> {
    Heightmap::from_str(&challenge_data)
}
