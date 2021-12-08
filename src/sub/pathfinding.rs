use micromath::vector::Vector2d;
use std::collections::HashMap;

pub struct LineSegment {
    p1: Vector2d<i32>,
    p2: Vector2d<i32>,
}

impl LineSegment {
    pub fn from((p1, p2): (Vector2d<i32>, Vector2d<i32>)) -> LineSegment {
        LineSegment { p1, p2 }
    }

    pub fn from_str(s: &str) -> Result<LineSegment, serde_scan::ScanError> {
        let (x1, y1, x2, y2): (i32, i32, i32, i32) = serde_scan::scan!("{},{} -> {},{}" <- s)?;
        Ok(LineSegment::from((
            Vector2d::from((x1, y1)),
            Vector2d::from((x2, y2)),
        )))
    }
}

#[derive(Debug)]
pub struct DangerMap {
    danger_coords: HashMap<i32, HashMap<i32, u32>>,
}

impl DangerMap {
    pub fn new() -> DangerMap {
        DangerMap {
            danger_coords: HashMap::new(),
        }
    }

    pub fn count_danger_above(&self, safe_value: u32) -> u32 {
        self.danger_coords.values().fold(0, |acc, y_map| {
            acc + y_map
                .values()
                .fold(0, |acc, val| if *val > safe_value { acc + 1 } else { acc })
        })
    }

    pub fn increment_danger(&mut self, at: Vector2d<i32>) {
        let y_map = self.mut_map_for_y(at.y);
        let current_danger = y_map.get(&at.x);
        let new_danger: u32 = current_danger.map_or(1, |danger| danger + 1);
        y_map.insert(at.x, new_danger);
    }

    fn mut_map_for_y(&mut self, y: i32) -> &mut HashMap<i32, u32> {
        if !self.danger_coords.contains_key(&y) {
            self.danger_coords.insert(y, HashMap::new());
        }
        self.danger_coords.get_mut(&y).unwrap()
    }
}

pub fn count_dangerous_points(vents: &Vec<LineSegment>) -> u32 {
    let mut danger_map = DangerMap::new();
    for vent in vents {
        if vent.p1.x == vent.p2.x {
            let x = vent.p1.x;
            let min_y = std::cmp::min(vent.p1.y, vent.p2.y);
            let max_y = std::cmp::max(vent.p1.y, vent.p2.y);
            for y in min_y..(max_y + 1) {
                danger_map.increment_danger(Vector2d::from((x, y)));
            }
        } else if vent.p1.y == vent.p2.y {
            let y = vent.p1.y;
            let min_x = std::cmp::min(vent.p1.x, vent.p2.x);
            let max_x = std::cmp::max(vent.p1.x, vent.p2.x);
            for x in min_x..(max_x + 1) {
                danger_map.increment_danger(Vector2d::from((x, y)));
            }
        }
    }
    danger_map.count_danger_above(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn can_create_segments_from_str() {
        let example_segments = get_example_segments();
        let segments_result: Result<Vec<LineSegment>, serde_scan::ScanError> = TEST_STR
            .lines()
            .map(|line| LineSegment::from_str(line))
            .collect();
        assert!(segments_result.is_ok());
        let segments = segments_result.unwrap();
        for (expected, actual) in example_segments.iter().zip(segments.iter()) {
            assert_eq!(expected.p1, actual.p1);
            assert_eq!(expected.p2, actual.p2);
        }
    }

    #[test]
    fn count_dangerous_points_works_for_example_data() {
        let example_segments = get_example_segments();
        let danger_count = count_dangerous_points(&example_segments);
        assert_eq!(5, danger_count);
    }

    fn get_example_segments() -> Vec<LineSegment> {
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
