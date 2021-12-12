mod component;
mod vector;

pub use component::Component;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use vector::Vector;

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

impl<C: Component + PartialOrd> PartialOrd for Vector2<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(ord_y) => match ord_y {
                Ordering::Less | Ordering::Greater => Some(ord_y),
                Ordering::Equal => match self.x.partial_cmp(&other.x) {
                    Some(ord_x) => Some(ord_x),
                    None => None,
                },
            },
            None => None,
        }
    }
}

impl<C: Component + PartialOrd + Ord> Ord for Vector2<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            ord @ (Ordering::Less | Ordering::Greater) => ord,
            Ordering::Equal => self.x.cmp(&other.x),
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        Self::from(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}

impl<C: Component + Hash> Hash for Vector2<C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
