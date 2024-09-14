use super::Primitive3d;
use crate::prelude::*;

/// A cylinder primitive
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Cylinder {
    /// The radius of the cylinder
    pub radius: f32,
    /// The half height of the cylinder
    pub half_height: f32,
}
impl Primitive3d for Cylinder {}

impl Default for Cylinder {
    /// Returns the default [`Cylinder`] with a radius of `0.5` and a height of `1.0`.
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_height: 0.5,
        }
    }
}

impl Cylinder {
    /// Create a new `Cylinder` from a radius and full height
    #[inline(always)]
    pub fn new(radius: f32, height: f32) -> Self {
        Self {
            radius,
            half_height: height / 2.0,
        }
    }

    /// Get the base of the cylinder as a [`Circle`]
    #[inline(always)]
    pub fn base(&self) -> Circle {
        Circle {
            radius: self.radius,
        }
    }

    /// Get the surface area of the side of the cylinder,
    /// also known as the lateral area
    #[inline(always)]
    #[doc(alias = "side_area")]
    pub fn lateral_area(&self) -> f32 {
        4.0 * PI * self.radius * self.half_height
    }

    /// Get the surface area of one base of the cylinder
    #[inline(always)]
    pub fn base_area(&self) -> f32 {
        PI * self.radius.powi(2)
    }
}

// impl Measured3d for Cylinder {
//     /// Get the total surface area of the cylinder
//     #[inline(always)]
//     fn area(&self) -> f32 {
//         2.0 * PI * self.radius * (self.radius + 2.0 * self.half_height)
//     }

//     /// Get the volume of the cylinder
//     #[inline(always)]
//     fn volume(&self) -> f32 {
//         self.base_area() * 2.0 * self.half_height
//     }
// }
