use super::{Primitive2d, WindingOrder};
use crate::prelude::*;

/// A triangle in 2D space
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Triangle2d {
    /// The vertices of the triangle
    pub vertices: [Vec2; 3],
}
impl Primitive2d for Triangle2d {}

impl Default for Triangle2d {
    /// Returns the default [`Triangle2d`] with the vertices `[0.0, 0.5]`, `[-0.5, -0.5]`, and `[0.5, -0.5]`.
    fn default() -> Self {
        Self {
            vertices: [Vec2::Y * 0.5, Vec2::new(-0.5, -0.5), Vec2::new(0.5, -0.5)],
        }
    }
}

impl Triangle2d {
    /// Create a new `Triangle2d` from points `a`, `b`, and `c`
    #[inline(always)]
    pub const fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    /// Get the area of the triangle
    #[inline(always)]
    pub fn area(&self) -> f32 {
        let [a, b, c] = self.vertices;
        (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)).abs() / 2.0
    }

    /// Get the perimeter of the triangle
    #[inline(always)]
    pub fn perimeter(&self) -> f32 {
        let [a, b, c] = self.vertices;

        let ab = a.distance(b);
        let bc = b.distance(c);
        let ca = c.distance(a);

        ab + bc + ca
    }

    /// Get the [`WindingOrder`] of the triangle
    #[inline(always)]
    #[doc(alias = "orientation")]
    pub fn winding_order(&self) -> WindingOrder {
        let [a, b, c] = self.vertices;
        let area = (b - a).perp_dot(c - a);
        if area > f32::EPSILON {
            WindingOrder::CounterClockwise
        } else if area < -f32::EPSILON {
            WindingOrder::Clockwise
        } else {
            WindingOrder::Invalid
        }
    }

    /// Reverse the [`WindingOrder`] of the triangle
    /// by swapping the second and third vertices
    #[inline(always)]
    pub fn reverse(&mut self) {
        self.vertices.swap(1, 2);
    }
}
