mod cursor;
mod value;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::repeat;
use std::num::ParseIntError;
use std::str::FromStr;

use super::vector::Vector2;
pub use cursor::Cursor;
pub use value::Value;

type Coords = Vector2<i64>;
type CursorFactory<T> = dyn Fn(&Map2D<T>, Vector2<i64>) -> Option<Cursor<T>>;

pub struct Map2D<T: Value> {
    data: Vec<Vec<T>>,
    cursor_factory: Box<CursorFactory<T>>,
}

impl<T: Value + FromStr<Err = ParseIntError>> FromStr for Map2D<T>
where
    Vec<T>: FromIterator<T>,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Map2D {
            data: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| T::from_str(&String::from(c)))
                        .collect::<std::result::Result<Vec<T>, ParseIntError>>()
                })
                .filter(|res| match res {
                    Ok(nums) => !nums.is_empty(),
                    Err(_) => true,
                })
                .collect::<std::result::Result<Vec<Vec<T>>, ParseIntError>>()?,
            cursor_factory: Box::new(|map2d, pos| {
                if pos.x < 0
                    || pos.x as u64 >= map2d.width()
                    || pos.y < 0
                    || pos.y as u64 >= map2d.height()
                {
                    None
                } else {
                    let mut cursor = Cursor::new(map2d);
                    cursor.position = pos;
                    Some(cursor)
                }
            }),
        })
    }
}

impl<T: Value> Map2D<T> {
    pub fn width(&self) -> u64 {
        match self.data.first() {
            Some(first) => first.len() as u64,
            None => 0,
        }
    }

    pub fn height(&self) -> u64 {
        self.data.len() as u64
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vector2<u64> {
        Vector2::from(self.width(), self.height())
    }

    pub fn get(&self, pos: Coords) -> Option<T> {
        match self.data.get(pos.y as usize) {
            Some(row) => match row.get(pos.x as usize) {
                Some(&val) => Some(val),
                None => None,
            },
            None => None,
        }
    }

    pub fn cursor_at(&self, pos: Vector2<i64>) -> Option<Cursor<T>> {
        (self.cursor_factory)(self, Vector2::from(pos.x, pos.y))
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (T, Coords)> + 'a> {
        Box::new(
            self.data
                .iter()
                .enumerate()
                .map(|(y_outer, row)| {
                    repeat(y_outer)
                        .zip(row.iter())
                        .enumerate()
                        .map(|(x, (y, &val))| (val, Vector2::from(x as i64, y as i64)))
                })
                .flatten(),
        )
    }
}

impl<T: 'static + Value> IntoIterator for Map2D<T> {
    type Item = (T, Coords);
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
                        .map(|(y, (x, val))| (val, Vector2::from(x as i64, y as i64)))
                })
                .flatten(),
        )
    }
}

impl<T: Value> Display for Map2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for val in row {
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
