mod hydrothermal_vents;

use hydrothermal_vents::DangerMap;
pub use hydrothermal_vents::LineSegment;

pub fn count_dangerous_points(vents: &Vec<LineSegment>, consider_diagonal_vents: bool) -> u32 {
    let mut danger_map = DangerMap::new();
    for vent in vents {
        if consider_diagonal_vents || !vent.is_diagonal() {
            for point in vent.line_points() {
                danger_map.increment_danger(point);
            }
        }
    }
    danger_map.count_danger_above(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use micromath::vector::Vector2d;

    pub const TEST_STR: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn count_dangerous_points_works_for_example_data_orthogonal() {
        let example_segments = get_example_segments();
        let danger_count = count_dangerous_points(&example_segments, false);
        assert_eq!(5, danger_count);
    }

    #[test]
    fn count_dangerous_points_works_for_example_data_diagonal() {
        let example_segments = get_example_segments();
        let danger_count = count_dangerous_points(&example_segments, true);
        assert_eq!(12, danger_count);
    }

    pub fn get_example_segments() -> Vec<LineSegment> {
        return vec![
            LineSegment::from((Vector2d::from((0, 9)), Vector2d::from((5, 9)))),
            LineSegment::from((Vector2d::from((8, 0)), Vector2d::from((0, 8)))),
            LineSegment::from((Vector2d::from((9, 4)), Vector2d::from((3, 4)))),
            LineSegment::from((Vector2d::from((2, 2)), Vector2d::from((2, 1)))),
            LineSegment::from((Vector2d::from((7, 0)), Vector2d::from((7, 4)))),
            LineSegment::from((Vector2d::from((6, 4)), Vector2d::from((2, 0)))),
            LineSegment::from((Vector2d::from((0, 9)), Vector2d::from((2, 9)))),
            LineSegment::from((Vector2d::from((3, 4)), Vector2d::from((1, 4)))),
            LineSegment::from((Vector2d::from((0, 0)), Vector2d::from((8, 8)))),
            LineSegment::from((Vector2d::from((5, 5)), Vector2d::from((8, 2)))),
        ];
    }
}
