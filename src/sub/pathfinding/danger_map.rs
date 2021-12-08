use micromath::vector::Vector2d;
use std::collections::HashMap;

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
