mod cursor;
mod value;

use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::{repeat, repeat_with};
use std::num::ParseIntError;
use std::str::FromStr;

use super::vector::Vector2;
pub use cursor::Cursor;
pub use value::Value;

type Coords = Vector2<i64>;
type CursorFactory<T> = dyn Fn(&Map2D<T>, Vector2<i64>) -> Option<Cursor<T>>;

pub struct Map2D<T: Value> {
    data: Vec<Vec<RefCell<T>>>,
    cursor_factory: Box<CursorFactory<T>>,
}

impl<T: Value> Map2D<T> {
    pub fn new(size: Vector2<u64>, initial_value: T) -> Self {
        Self {
            data: repeat_with(|| {
                repeat_with(|| RefCell::new(initial_value))
                    .take(size.x as usize)
                    .collect::<Vec<RefCell<T>>>()
            })
            .take(size.y as usize)
            .collect(),
            cursor_factory: Self::cursor_factory_default(),
        }
    }

    fn cursor_factory_default() -> Box<CursorFactory<T>> {
        Box::new(|map2d, pos| {
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
        })
    }
}

impl<T: Value + FromStr<Err = ParseIntError>> FromStr for Map2D<T>
where
    Vec<T>: FromIterator<T>,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data_values = s
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
            .collect::<std::result::Result<Vec<Vec<T>>, ParseIntError>>()?;
        Ok(Map2D {
            data: data_values
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|val| RefCell::new(val))
                        .collect::<Vec<RefCell<T>>>()
                })
                .collect(),
            cursor_factory: Self::cursor_factory_default(),
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
        match self.get_ref_cell(pos) {
            Some(ref_cell) => Some(*ref_cell.borrow()),
            None => None,
        }
    }

    fn get_ref_cell(&self, pos: Coords) -> Option<&RefCell<T>> {
        match self.data.get(pos.y as usize) {
            Some(row) => match row.get(pos.x as usize) {
                Some(ref_cell) => Some(ref_cell),
                None => None,
            },
            None => None,
        }
    }

    pub fn set(&mut self, pos: Coords, value: T) -> bool {
        let i_col = pos.x as usize;
        let i_row = pos.y as usize;
        if i_row < self.data.len() && i_col < self.data[i_row].len() {
            self.data[i_row][i_col].replace(value);
            return true;
        }
        false
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
                        .map(|(x, (y, ref_cell))| {
                            (*ref_cell.borrow(), Vector2::from(x as i64, y as i64))
                        })
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
                        .map(|(y, (x, ref_cell))| {
                            (*ref_cell.borrow(), Vector2::from(x as i64, y as i64))
                        })
                })
                .flatten(),
        )
    }
}

impl<T: Value> Display for Map2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for ref_cell in row {
                write!(f, "{}", *ref_cell.borrow())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
