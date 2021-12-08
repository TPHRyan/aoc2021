use crate::sub::pathfinding::{self, LineSegment};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let vents: Vec<LineSegment> = challenge_data
        .lines()
        .map(|line| LineSegment::from_str(line))
        .collect::<std::result::Result<Vec<LineSegment>, serde_scan::ScanError>>()?;
    let dangerous_point_count = pathfinding::count_dangerous_points(&vents);
    println!("Number of dangerous points: {}", dangerous_point_count);
    Ok(())
}

pub fn solve_part_2(_challenge_data: String) -> Result<()> {
    Ok(())
}
