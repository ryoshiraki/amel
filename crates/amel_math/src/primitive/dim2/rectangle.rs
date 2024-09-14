use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Rectangle {
    pub size: Vec2,
}
impl Primitive2d for Rectangle {}

impl Default for Rectangle {
    /// Returns the default [`Rectangle`] with a half-width and half-height of `0.5`.
    fn default() -> Self {
        Self {
            size: Vec2::splat(1.0),
        }
    }
}

impl Rectangle {
    /// Create a new `Rectangle` from a full width and height
    #[inline(always)]
    pub fn new(width: f32, height: f32) -> Self {
        Self::from_size(Vec2::new(width, height))
    }

    /// Create a new `Rectangle` from a given full size
    #[inline(always)]
    pub fn from_size(size: Vec2) -> Self {
        Self { size }
    }

    /// Create a new `Rectangle` from two corner points
    #[inline(always)]
    pub fn from_corners(point1: Vec2, point2: Vec2) -> Self {
        Self {
            size: (point2 - point1).abs(),
        }
    }

    /// Create a `Rectangle` from a single length.
    /// The resulting `Rectangle` will be the same size in every direction.
    #[inline(always)]
    pub fn from_length(length: f32) -> Self {
        Self {
            size: Vec2::splat(length),
        }
    }

    /// Get the size of the rectangle
    #[inline(always)]
    pub fn size(&self) -> Vec2 {
        self.size
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.size.x
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.size.y
    }

    /// Get the area of the rectangle
    #[inline(always)]
    pub fn area(&self) -> f32 {
        self.size.x * self.size.y
    }

    /// Get the perimeter of the rectangle
    #[inline(always)]
    pub fn perimeter(&self) -> f32 {
        2.0 * (self.size.x + self.size.y)
    }

    /// Finds the point on the rectangle that is closest to the given `point`.
    ///
    /// If the point is outside the rectangle, the returned point will be on the perimeter of the rectangle.
    /// Otherwise, it will be inside the rectangle and returned as is.
    #[inline(always)]
    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        // Clamp point coordinates to the rectangle
        let half_size = self.size / 2.0;
        point.clamp(-half_size, half_size)
    }
}
