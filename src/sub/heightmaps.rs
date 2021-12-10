mod heightmap;

use std::collections::HashMap;

use crate::common::Vector2;
pub use heightmap::Heightmap;

pub fn find_valleys(heightmap: &Heightmap) -> Vec<Vector2<u32>> {
    let mut valleys = vec![];
    'next_position: for (val, pos) in heightmap.iter() {
        for [x_dir, y_dir] in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
            if heightmap.neighbour_value(pos, Vector2::from(x_dir, y_dir)) <= val {
                continue 'next_position;
            }
        }
        valleys.push(pos);
    }
    valleys
}

pub fn get_risk_levels(heightmap: &Heightmap) -> HashMap<Vector2<u32>, u8> {
    let mut risk_levels: HashMap<Vector2<u32>, u8> = HashMap::new();
    for pos in find_valleys(&heightmap) {
        if let Some(val) = heightmap.get(pos) {
            risk_levels.insert(pos, val + 1);
        }
    }
    risk_levels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    const TEST_STR: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn parsing_heightmap_works() -> Result<()> {
        let parsed_heightmap = Heightmap::from_str(TEST_STR)?;
        assert_eq!(Vector2::from(10, 5), parsed_heightmap.size());
        Ok(())
    }

    #[test]
    fn find_valleys_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let expected_valleys: Vec<Vector2<u32>> = vec![
            Vector2::from(1, 0),
            Vector2::from(9, 0),
            Vector2::from(2, 2),
            Vector2::from(6, 4),
        ];
        let valleys = find_valleys(&example_heightmap);
        assert_eq!(expected_valleys.len(), valleys.len());
        for (&expected_valley, actual_valley) in expected_valleys.iter().zip(valleys) {
            assert_eq!(expected_valley, actual_valley);
        }
        Ok(())
    }

    #[test]
    fn get_risk_levels_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let risk_levels = get_risk_levels(&example_heightmap);
        let expected_risk_levels: Vec<(Vector2<u32>, u8)> = vec![
            (Vector2::from(1, 0), 2),
            (Vector2::from(9, 0), 1),
            (Vector2::from(2, 2), 6),
            (Vector2::from(6, 4), 6),
        ];
        assert_eq!(expected_risk_levels.len(), risk_levels.len());
        for (pos, expected_risk) in expected_risk_levels {
            assert_eq!(expected_risk, risk_levels[&pos]);
        }
        Ok(())
    }
}
