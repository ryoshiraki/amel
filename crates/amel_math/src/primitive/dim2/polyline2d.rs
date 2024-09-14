use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline2d<const N: usize> {
    /// The vertices of the polyline
    #[cfg_attr(feature = "serialize", serde(with = "super::serde::array"))]
    pub vertices: [Vec2; N],
}
impl<const N: usize> Primitive2d for Polyline2d<N> {}

impl<const N: usize> FromIterator<Vec2> for Polyline2d<N> {
    fn from_iter<I: IntoIterator<Item = Vec2>>(iter: I) -> Self {
        let mut vertices: [Vec2; N] = [Vec2::ZERO; N];

        for (index, i) in iter.into_iter().take(N).enumerate() {
            vertices[index] = i;
        }
        Self { vertices }
    }
}

impl<const N: usize> Polyline2d<N> {
    /// Create a new `Polyline2d` from its vertices
    pub fn new(vertices: impl IntoIterator<Item = Vec2>) -> Self {
        Self::from_iter(vertices)
    }

    pub fn count(&self) -> usize {
        N
    }
}
