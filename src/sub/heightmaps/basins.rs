use crate::common::map2d::Cursor;
use std::collections::HashMap;

use super::find_valleys;
use super::heightmap::Heightmap;
use crate::common::Vector2;

type Coords = Vector2<i64>;

pub fn find_basins(heightmap: &Heightmap) -> HashMap<Coords, Vec<Coords>> {
    let origin_points = find_valleys(heightmap);
    let basins: Vec<Vec<Coords>> = origin_points
        .iter()
        .map(|valley| vec![valley.clone()])
        .collect();
    fully_expand_basins(heightmap, basins, origin_points)
}

fn fully_expand_basins(
    heightmap: &Heightmap,
    mut basins: Vec<Vec<Coords>>,
    origin_points: Vec<Coords>,
) -> HashMap<Coords, Vec<Coords>> {
    let mut final_basins: Vec<Vec<Coords>> = vec![];
    for _ in 0..basins.len() {
        final_basins.push(vec![]);
    }
    let mut visited: HashMap<Coords, ()> = HashMap::new();
    let mut got_new_positions = true;
    while got_new_positions {
        let mut new_final_positions = expand_basins(heightmap, &mut basins, &mut visited);
        got_new_positions = new_final_positions
            .iter()
            .fold(false, |got_final, basin_positions| {
                got_final || !basin_positions.is_empty()
            });
        new_final_positions
            .iter_mut()
            .zip(final_basins.iter_mut())
            .for_each(|(from, to)| to.append(from));
    }
    collect_basins_into_map(final_basins, &origin_points)
}

fn expand_basins(
    heightmap: &Heightmap,
    basins: &mut Vec<Vec<Coords>>,
    visited: &mut HashMap<Coords, ()>,
) -> Vec<Vec<Coords>> {
    let mut final_positions = vec![];
    for basin in basins.iter_mut() {
        final_positions.push(expand_basin(heightmap, basin, visited));
    }
    final_positions
}

fn expand_basin(
    heightmap: &Heightmap,
    basin: &mut Vec<Coords>,
    visited: &mut HashMap<Coords, ()>,
) -> Vec<Coords> {
    let mut new_positions = vec![];
    let mut final_positions = vec![];
    while let Some(pos) = basin.pop() {
        if visited.contains_key(&pos) {
            continue;
        } else {
            final_positions.push(pos);
            visited.insert(pos, ());
        }
        let cursor_at_pos = heightmap.cursor_at(pos).expect(&format!(
            "Invalid position for cursor encountered: ({}, {})",
            pos.x, pos.y
        ));
        let cursors: Vec<Option<Cursor<u8>>> = vec![
            cursor_at_pos.left(),
            cursor_at_pos.up(),
            cursor_at_pos.right(),
            cursor_at_pos.down(),
        ];
        for cursor in cursors.into_iter().filter_map(|c| c) {
            if cursor.value() < 9 && !visited.contains_key(&cursor.position) {
                new_positions.push(cursor.position);
            }
        }
    }
    basin.append(&mut new_positions);
    final_positions
}

fn collect_basins_into_map(
    basins: Vec<Vec<Coords>>,
    origin_points: &Vec<Coords>,
) -> HashMap<Coords, Vec<Coords>> {
    basins
        .into_iter()
        .enumerate()
        .filter_map(|(i, points)| match origin_points.get(i) {
            Some(&pos) => Some((pos, points)),
            None => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use std::str::FromStr;

    const TEST_STR: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn find_basins_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let basins_map = find_basins(&example_heightmap);
        let expected_basin_sizes: Vec<(Coords, usize)> = vec![
            (Vector2::from(1, 0), 3),
            (Vector2::from(9, 0), 9),
            (Vector2::from(2, 2), 14),
            (Vector2::from(6, 4), 9),
        ];
        assert_eq!(expected_basin_sizes.len(), basins_map.len());
        for (pos, expected_size) in expected_basin_sizes {
            assert_eq!(expected_size, basins_map[&pos].len());
        }
        Ok(())
    }
}
