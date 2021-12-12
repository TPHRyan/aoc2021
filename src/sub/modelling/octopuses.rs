use crate::common::{Map2D, Vector2};
use std::error::Error;
use std::str::FromStr;

type Coords = Vector2<i64>;

pub struct Octopuses {
    map2d: Map2D<u8>,
    flashes: Map2D<bool>,
    total_flash_count: u64,
    last_flash_count: u64,
}

impl FromStr for Octopuses {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Octopuses, Self::Err> {
        let main_map = Map2D::from_str(s)?;
        let flash_map = Map2D::new(main_map.size(), false);
        Ok(Octopuses {
            map2d: main_map,
            flashes: flash_map,
            total_flash_count: 0,
            last_flash_count: 0,
        })
    }
}

impl Octopuses {
    pub fn simulate(&mut self, n_ticks: u64) -> u64 {
        for _ in 0..n_ticks {
            self.last_flash_count = self.tick();
            self.total_flash_count += self.last_flash_count;
        }
        self.total_flash_count
    }

    pub fn tick(&mut self) -> u64 {
        let initial_flashes = self.increase_all_energy();
        self.cascade_flashes(initial_flashes);
        self.unflash_all()
    }

    fn increase_all_energy(&mut self) -> Vec<Coords> {
        let previous_state: Vec<(u8, Coords)> = self.map2d.iter().collect();
        let mut flashes = vec![];
        for (octopus, position) in previous_state {
            self.map2d.set(position, octopus + 1);
            if octopus + 1 > 9
                && !self.flashes.get(position).expect(&format!(
                    "Flash state should be set at position ({}, {})",
                    position.x, position.y
                ))
            {
                self.flashes.set(position, true);
                flashes.push(position);
            }
        }
        flashes
    }

    fn cascade_flashes(&mut self, initial_flashes: Vec<Coords>) {
        let mut flashes = Vec::from_iter(initial_flashes.into_iter());
        while !flashes.is_empty() {
            let mut new_flashes = vec![];
            while let Some(flash_pos) = flashes.pop() {
                let flash_cursor = self.map2d.cursor_at(flash_pos).expect(&format!(
                    "Cursor could not be retrieved at position that flash occurred! ({}, {})",
                    flash_pos.x, flash_pos.y
                ));
                let cursors = vec![
                    flash_cursor.left(),
                    flash_cursor.up_left(),
                    flash_cursor.up(),
                    flash_cursor.up_right(),
                    flash_cursor.right(),
                    flash_cursor.down_right(),
                    flash_cursor.down(),
                    flash_cursor.down_left(),
                ];
                for mut cursor in cursors.into_iter().filter_map(|c| c) {
                    let current_value = cursor.value();
                    cursor.value_set(current_value + 1);
                    if current_value + 1 > 9
                        && !self.flashes.get(cursor.position).expect(&format!(
                            "Flash state should be set at position ({}, {})",
                            cursor.position.x, cursor.position.y
                        ))
                    {
                        self.flashes.set(cursor.position, true);
                        new_flashes.push(cursor.position);
                    }
                }
            }
            flashes.append(&mut new_flashes);
        }
    }

    fn unflash_all(&mut self) -> u64 {
        let flashes: Vec<Coords> = self
            .flashes
            .iter()
            .filter_map(|(flashing, pos)| if flashing { Some(pos) } else { None })
            .collect();
        let flash_count = flashes.len();
        for pos in flashes {
            self.map2d.set(pos, 0);
            self.flashes.set(pos, false);
        }
        flash_count as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn simulation_correct_for_example_1() -> Result<()> {
        let initial_state = "
11111
19991
19191
19991
11111
";
        let mut octopuses = Octopuses::from_str(initial_state)?;

        octopuses.tick();
        assert_row_sums(vec![19, 8, 10, 8, 19], &octopuses);

        octopuses.tick();
        assert_row_sums(vec![24, 13, 15, 13, 24], &octopuses);
        Ok(())
    }

    fn assert_row_sums(expected_row_sums: Vec<u64>, octopuses: &Octopuses) {
        for (expected_sum, actual_sum) in expected_row_sums.into_iter().zip(row_sums(octopuses)) {
            assert_eq!(expected_sum, actual_sum)
        }
    }

    fn row_sums(octopuses: &Octopuses) -> Vec<u64> {
        let mut row_sums = vec![];
        for row in 0..octopuses.map2d.height() {
            let row_cursor = octopuses
                .map2d
                .cursor_at(Vector2::from(0, row as i64))
                .unwrap();
            let mut current_cursor = row_cursor;
            let mut sum = row_cursor.value() as u64;
            while let Some(cursor) = current_cursor.right() {
                current_cursor = cursor;
                sum += cursor.value() as u64;
            }
            row_sums.push(sum);
        }
        row_sums
    }
}
