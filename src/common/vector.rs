pub use micromath::vector::Component;
use micromath::vector::Vector;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Vector2<C: Component> {
    pub x: C,
    pub y: C,
}

impl<C: Component> Vector2<C> {
    pub fn from(x: C, y: C) -> Vector2<C> {
        Vector2 { x, y }
    }
}

impl<C: Component> FromIterator<C> for Vector2<C> {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let x = iter.next().expect("no x-axis component in slice");
        let y = iter.next().expect("no y-axis component in slice");

        assert!(
            iter.next().is_none(),
            "too many items for 2-dimensional vector"
        );

        Self::from(x, y)
    }
}

impl<C: Component> Vector<C> for Vector2<C> {
    const AXES: usize = 2;

    fn get(self, index: usize) -> Option<C> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }

    fn dot(self, rhs: Self) -> C {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl<C: Component + Hash> Hash for Vector2<C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
