use super::value::Value;
use super::Map2D;
use crate::common::Vector2;

#[derive(Copy, Clone)]
pub struct Cursor<'m, T: Value> {
    map2d: &'m Map2D<T>,
    pub position: Vector2<i64>,
}

impl<'m, T: Value> Cursor<'m, T> {
    pub fn new(map2d: &Map2D<T>) -> Cursor<T> {
        Cursor {
            map2d,
            position: Vector2::default(),
        }
    }

    pub fn value(&self) -> T {
        self.map2d
            .get(self.position)
            .expect("map2d::Cursor should always be at valid coordinates!")
    }

    pub fn left(&self) -> Option<Self> {
        self.new_cursor(Vector2::<i64>::from(
            self.position.x as i64 - 1,
            self.position.y as i64,
        ))
    }

    pub fn up(&self) -> Option<Self> {
        self.new_cursor(Vector2::<i64>::from(
            self.position.x as i64,
            self.position.y as i64 - 1,
        ))
    }

    pub fn right(&self) -> Option<Self> {
        self.new_cursor(Vector2::<i64>::from(
            self.position.x as i64 + 1,
            self.position.y as i64,
        ))
    }

    pub fn down(&self) -> Option<Self> {
        self.new_cursor(Vector2::<i64>::from(
            self.position.x as i64,
            self.position.y as i64 + 1,
        ))
    }

    fn new_cursor(&self, pos: Vector2<i64>) -> Option<Cursor<'m, T>> {
        self.map2d.cursor_at(pos)
    }
}
