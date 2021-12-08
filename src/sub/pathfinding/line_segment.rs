use micromath::vector::Vector2d;

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

    pub fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn line_points(&self) -> Box<dyn Iterator<Item = Vector2d<i32>>> {
        if self.is_horizontal() {
            self.line_points_horizontal()
        } else if self.is_vertical() {
            self.line_points_vertical()
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn line_points_horizontal(&self) -> Box<dyn Iterator<Item = Vector2d<i32>>> {
        let y = self.p1.y;
        let min_x = std::cmp::min(self.p1.x, self.p2.x);
        let max_x = std::cmp::max(self.p1.x, self.p2.x);
        Box::new((min_x..(max_x + 1)).map(move |x| Vector2d::from((x, y))))
    }

    fn line_points_vertical(&self) -> Box<dyn Iterator<Item = Vector2d<i32>>> {
        let x = self.p1.x;
        let min_y = std::cmp::min(self.p1.y, self.p2.y);
        let max_y = std::cmp::max(self.p1.y, self.p2.y);
        Box::new((min_y..(max_y + 1)).map(move |y| Vector2d::from((x, y))))
    }
}
