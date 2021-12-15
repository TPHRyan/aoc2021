use crate::sub::pathfinding::CaveSystem;
use crate::Result;
use std::str::FromStr;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let cave_system = CaveSystem::from_str(&challenge_data)?;
    let paths = cave_system.find_paths("start", "end", false);
    println!("There are {} paths from start to end.", paths.len());
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let cave_system = CaveSystem::from_str(&challenge_data)?;
    let paths = cave_system.find_paths("start", "end", true);
    println!("There are {} paths from start to end.", paths.len());
    Ok(())
}
