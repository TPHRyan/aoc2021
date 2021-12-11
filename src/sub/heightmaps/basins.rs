use std::collections::HashMap;

use super::find_valleys;
use super::heightmap::Heightmap;
use crate::common::Vector2;

pub fn find_basins(heightmap: &Heightmap) -> HashMap<Vector2<u32>, Vec<Vector2<u32>>> {
    let origin_points = find_valleys(heightmap);
    let basins: Vec<Vec<Vector2<u32>>> = origin_points
        .iter()
        .map(|valley| vec![valley.clone()])
        .collect();
    fully_expand_basins(heightmap, basins, origin_points)
}

fn fully_expand_basins(
    heightmap: &Heightmap,
    mut basins: Vec<Vec<Vector2<u32>>>,
    origin_points: Vec<Vector2<u32>>,
) -> HashMap<Vector2<u32>, Vec<Vector2<u32>>> {
    let mut final_basins: Vec<Vec<Vector2<u32>>> = vec![];
    for _ in 0..basins.len() {
        final_basins.push(vec![]);
    }
    let mut visited: HashMap<Vector2<u32>, ()> = HashMap::new();
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
    basins: &mut Vec<Vec<Vector2<u32>>>,
    visited: &mut HashMap<Vector2<u32>, ()>,
) -> Vec<Vec<Vector2<u32>>> {
    let mut final_positions = vec![];
    for basin in basins.iter_mut() {
        final_positions.push(expand_basin(heightmap, basin, visited));
    }
    final_positions
}

fn expand_basin(
    heightmap: &Heightmap,
    basin: &mut Vec<Vector2<u32>>,
    visited: &mut HashMap<Vector2<u32>, ()>,
) -> Vec<Vector2<u32>> {
    let mut new_positions = vec![];
    let mut final_positions = vec![];
    while let Some(pos) = basin.pop() {
        if visited.contains_key(&pos) {
            continue;
        } else {
            final_positions.push(pos);
            visited.insert(pos, ());
        }
        for [dir_x, dir_y] in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
            if dir_x < 0 && pos.x == 0 || dir_y < 0 && pos.y == 0 {
                continue;
            }
            let new_pos = Vector2::from((pos.x as i8 + dir_x) as u32, (pos.y as i8 + dir_y) as u32);
            if heightmap.neighbour_value(pos, Vector2::from(dir_x, dir_y)) < 9
                && !visited.contains_key(&new_pos)
            {
                new_positions.push(new_pos);
            }
        }
    }
    basin.append(&mut new_positions);
    final_positions
}

fn collect_basins_into_map(
    basins: Vec<Vec<Vector2<u32>>>,
    origin_points: &Vec<Vector2<u32>>,
) -> HashMap<Vector2<u32>, Vec<Vector2<u32>>> {
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

    const TEST_STR: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn find_basins_works_on_example_data() -> Result<()> {
        let example_heightmap = Heightmap::from_str(TEST_STR)?;
        let basins_map = find_basins(&example_heightmap);
        let expected_basin_sizes: Vec<(Vector2<u32>, usize)> = vec![
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
