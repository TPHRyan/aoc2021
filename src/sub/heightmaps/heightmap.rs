use std::iter::repeat;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::common::Vector2;
use crate::Result;

pub struct Heightmap {
    data: Vec<Vec<u8>>,
}

impl Heightmap {
    pub fn from_str(s: &str) -> Result<Heightmap> {
        let data: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| u8::from_str(&String::from(c)))
                    .collect::<std::result::Result<Vec<u8>, ParseIntError>>()
            })
            .filter(|res| match res {
                Ok(nums) => !nums.is_empty(),
                Err(_) => true,
            })
            .collect::<std::result::Result<Vec<Vec<u8>>, ParseIntError>>()?;
        Ok(Heightmap { data })
    }

    #[cfg(test)]
    pub fn size(&self) -> Vector2<u32> {
        match self.data.first() {
            Some(first) => Vector2::from(first.len() as u32, self.data.len() as u32),
            None => Vector2::from(0, 0),
        }
    }

    pub fn get(&self, pos: Vector2<u32>) -> Option<u8> {
        match self.data.get(pos.y as usize) {
            Some(row) => match row.get(pos.x as usize) {
                Some(&val) => Some(val),
                None => None,
            },
            None => None,
        }
    }

    pub fn neighbour_value(&self, pos: Vector2<u32>, direction: Vector2<i8>) -> u8 {
        let delta_x: i32 = if direction.x > 0 {
            1
        } else if direction.x < 0 {
            -1
        } else {
            0
        };
        let try_col = pos.x as i32 + delta_x;
        let delta_y: i32 = if direction.y > 0 {
            1
        } else if direction.y < 0 {
            -1
        } else {
            0
        };
        let try_row = pos.y as i32 + delta_y;

        if try_row < 0
            || try_row as usize >= self.data.len()
            || try_col < 0
            || try_col as usize >= self.data[try_row as usize].len()
        {
            9
        } else {
            self.data[try_row as usize][try_col as usize]
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (u8, Vector2<u32>)> + 'a> {
        Box::new(
            self.data
                .iter()
                .enumerate()
                .map(|(y_outer, row)| {
                    repeat(y_outer)
                        .zip(row.iter())
                        .enumerate()
                        .map(|(x, (y, &val))| (val, Vector2::from(x as u32, y as u32)))
                })
                .flatten(),
        )
    }
}

impl IntoIterator for Heightmap {
    type Item = (u8, Vector2<u32>);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.data
                .into_iter()
                .enumerate()
                .map(|(y_outer, row)| {
                    repeat(y_outer)
                        .zip(row.into_iter())
                        .enumerate()
                        .map(|(y, (x, val))| (val, Vector2::from(x as u32, y as u32)))
                })
                .flatten(),
        )
    }
}
