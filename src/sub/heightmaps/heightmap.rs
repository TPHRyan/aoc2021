use std::error::Error;
use std::str::FromStr;

use crate::common::map2d;
use crate::common::Vector2;

type Coords = Vector2<i64>;

pub struct Heightmap {
    map2d: map2d::Map2D<u8>,
}

impl FromStr for Heightmap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Heightmap, Self::Err> {
        Ok(Heightmap {
            map2d: map2d::Map2D::from_str(s)?,
        })
    }
}

impl Heightmap {
    #[cfg(test)]
    pub fn size(&self) -> Vector2<u64> {
        self.map2d.size()
    }

    pub fn get(&self, pos: Coords) -> Option<u8> {
        self.map2d.get(pos)
    }

    pub fn cursor_at(&self, pos: Vector2<i64>) -> Option<map2d::Cursor<u8>> {
        self.map2d.cursor_at(pos)
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (u8, Coords)> + 'a> {
        self.map2d.iter()
    }
}

impl IntoIterator for Heightmap {
    type Item = (u8, Coords);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map2d.into_iter()
    }
}
