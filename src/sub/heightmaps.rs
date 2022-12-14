pub mod basins;
mod heightmap;

use crate::common::Vector2;
pub use heightmap::Heightmap;
use std::collections::HashMap;

type Coords = Vector2<i64>;

pub fn find_valleys(heightmap: &Heightmap) -> Vec<Coords> {
    let mut valleys = vec![];
    'next_position: for (val, pos) in heightmap.iter() {
        let cursor_at_pos = heightmap.cursor_at(pos).expect(&format!(
            "Invalid position for cursor encountered: ({}, {})",
            pos.x, pos.y
        ));
        let cursors = vec![
            cursor_at_pos.left(),
            cursor_at_pos.up(),
            cursor_at_pos.right(),
            cursor_at_pos.down(),
        ];
        for cursor in cursors.into_iter().filter_map(|c| c) {
            if cursor.value() <= val {
                continue 'next_position;
            }
        }
        valleys.push(pos);
    }
    valleys
}

pub fn get_risk_levels(heightmap: &Heightmap) -> HashMap<Coords, u8> {
    let mut risk_levels: HashMap<Coords, u8> = HashMap::new();
    for pos in find_valleys(&heightmap) {
        if let Some(val) = heightmap.get(pos) {
            risk_levels.insert(pos, val + 1);
        }
    }
    risk_levels
}

pub fn get_basin_sizes(heightmap: &Heightmap, sorted: bool) -> Vec<usize> {
    let basins = basins::find_basins(heightmap);
    let mut basin_sizes: Vec<usize> = basins.values().map(|basin| basin.len()).collect();
    if sorted {
        basin_sizes.sort_unstable();
    }
    basin_sizes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use std::str::FromStr;

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
        let expected_valleys: Vec<Coords> = vec![
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
    fn find_basin_sizes_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let basin_sizes = get_basin_sizes(&example_heightmap, true);
        let expected_sizes: Vec<usize> = vec![3, 9, 9, 14];
        assert_eq!(expected_sizes.len(), basin_sizes.len());
        for (&expected_size, actual_size) in expected_sizes.iter().zip(basin_sizes) {
            assert_eq!(expected_size, actual_size);
        }
        Ok(())
    }

    #[test]
    fn get_risk_levels_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let risk_levels = get_risk_levels(&example_heightmap);
        let expected_risk_levels: Vec<(Coords, u8)> = vec![
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
