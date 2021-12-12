mod iter;

use std::fmt::Debug;

use super::component::Component;
use iter::Iter;

/// Algebraic vector generic over a given [`Component`] type.
pub trait Vector<C>: Copy + Debug + Default + FromIterator<C> + Send + Sync
where
    C: Component,
{
    /// Number of axes
    const AXES: usize;

    /// Get the component value for a particular index
    fn get(self, index: usize) -> Option<C>;

    /// Compute the dot product of two vectors
    fn dot(self, rhs: Self) -> C;

    /// Instantiate a vector from a slice of components.
    ///
    /// Panics if the slice is not the right size.
    fn from_slice(slice: &[C]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    /// Iterate over the components of this vector
    fn iter(&self) -> Iter<'_, Self, C> {
        Iter::new(self)
    }

    /// Compute the distance between two vectors
    fn distance(self, rhs: Self) -> f32
    where
        C: Into<f32>,
    {
        let differences = self
            .iter()
            .zip(rhs.iter())
            .map(|(a, b)| a.into() - b.into());

        differences.map(|n| n * n).sum::<f32>().sqrt()
    }

    /// Compute the magnitude of a vector
    fn magnitude(self) -> f32
    where
        C: Into<f32>,
    {
        self.iter()
            .map(|n| {
                let n = n.into();
                n * n
            })
            .sum::<f32>()
            .sqrt()
    }
}
