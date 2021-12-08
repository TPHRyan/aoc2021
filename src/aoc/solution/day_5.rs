use crate::sub::pathfinding::{self, LineSegment};
use crate::Result;

pub fn solve_part_1(challenge_data: String) -> Result<()> {
    let vents = parse_vents(&challenge_data)?;
    let dangerous_point_count = pathfinding::count_dangerous_points(&vents, false);
    println!("Number of dangerous points: {}", dangerous_point_count);
    Ok(())
}

pub fn solve_part_2(challenge_data: String) -> Result<()> {
    let vents = parse_vents(&challenge_data)?;
    let dangerous_point_count = pathfinding::count_dangerous_points(&vents, true);
    println!("Number of dangerous points: {}", dangerous_point_count);
    Ok(())
}

fn parse_vents(s: &str) -> std::result::Result<Vec<LineSegment>, serde_scan::ScanError> {
    s.lines().map(|line| LineSegment::from_str(line)).collect()
}
