use micromath::vector::Vector2d;
use std::iter::{repeat, Map, Repeat, Zip};

pub struct LineSegment {
    pub p1: Vector2d<i32>,
    pub p2: Vector2d<i32>,
}

impl LineSegment {
    pub fn from((p1, p2): (Vector2d<i32>, Vector2d<i32>)) -> LineSegment {
        LineSegment { p1, p2 }
    }

    pub fn from_str(s: &str) -> Result<LineSegment, serde_scan::ScanError> {
        let (x1, y1, x2, y2): (i32, i32, i32, i32) = serde_scan::scan!("{},{} -> {},{}" <- s)?;
        Ok(LineSegment::from((
            Vector2d::from((x1, y1)),
            Vector2d::from((x2, y2)),
        )))
    }

    pub fn is_diagonal(&self) -> bool {
        self.p1.x != self.p2.x && self.p1.y != self.p2.y
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn line_points(&self) -> Box<dyn Iterator<Item = Vector2d<i32>>> {
        return if self.is_horizontal() {
            Box::new(self.line_points_horizontal())
        } else if self.is_vertical() {
            Box::new(self.line_points_vertical())
        } else {
            Box::new(self.line_points_diagonal())
        };
    }

    fn line_points_horizontal(
        &self,
    ) -> Map<Zip<Box<dyn Iterator<Item = i32>>, Repeat<i32>>, fn((i32, i32)) -> Vector2d<i32>> {
        let xy_iter = (range_iter(self.p1.x, self.p2.x)).zip(repeat(self.p1.y));
        xy_iter.map(|(x, y)| Vector2d::from((x, y)))
    }

    fn line_points_vertical(
        &self,
    ) -> Map<Zip<Repeat<i32>, Box<dyn Iterator<Item = i32>>>, fn((i32, i32)) -> Vector2d<i32>> {
        let xy_iter = repeat(self.p1.x).zip(range_iter(self.p1.y, self.p2.y));
        xy_iter.map(|(x, y)| Vector2d::from((x, y)))
    }

    fn line_points_diagonal(
        &self,
    ) -> Map<
        Zip<Box<dyn Iterator<Item = i32>>, Box<dyn Iterator<Item = i32>>>,
        fn((i32, i32)) -> Vector2d<i32>,
    > {
        let xy_iter = (range_iter(self.p1.x, self.p2.x)).zip(range_iter(self.p1.y, self.p2.y));
        xy_iter.map(|(x, y)| Vector2d::from((x, y)))
    }
}

fn range_iter(from: i32, to: i32) -> Box<dyn Iterator<Item = i32>> {
    if from < to {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    }
}
