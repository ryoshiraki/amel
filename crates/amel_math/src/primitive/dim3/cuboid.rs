use super::Primitive3d;
use crate::prelude::*;

/// A cuboid primitive, more commonly known as a box.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Cuboid {
    /// Half of the width, height and depth of the cuboid
    pub half_size: Vec3,
}
impl Primitive3d for Cuboid {}

impl Default for Cuboid {
    /// Returns the default [`Cuboid`] with a width, height, and depth of `1.0`.
    fn default() -> Self {
        Self {
            half_size: Vec3::splat(0.5),
        }
    }
}

impl Cuboid {
    /// Create a new `Cuboid` from a full x, y, and z length
    #[inline(always)]
    pub fn new(x_length: f32, y_length: f32, z_length: f32) -> Self {
        Self::from_size(Vec3::new(x_length, y_length, z_length))
    }

    /// Create a new `Cuboid` from a given full size
    #[inline(always)]
    pub fn from_size(size: Vec3) -> Self {
        Self {
            half_size: size / 2.0,
        }
    }

    /// Create a new `Cuboid` from two corner points
    #[inline(always)]
    pub fn from_corners(point1: Vec3, point2: Vec3) -> Self {
        Self {
            half_size: (point2 - point1).abs() / 2.0,
        }
    }

    /// Create a `Cuboid` from a single length.
    /// The resulting `Cuboid` will be the same size in every direction.
    #[inline(always)]
    pub fn from_length(length: f32) -> Self {
        Self {
            half_size: Vec3::splat(length / 2.0),
        }
    }

    /// Get the size of the cuboid
    #[inline(always)]
    pub fn size(&self) -> Vec3 {
        2.0 * self.half_size
    }

    /// Finds the point on the cuboid that is closest to the given `point`.
    ///
    /// If the point is outside the cuboid, the returned point will be on the surface of the cuboid.
    /// Otherwise, it will be inside the cuboid and returned as is.
    #[inline(always)]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        // Clamp point coordinates to the cuboid
        point.clamp(-self.half_size, self.half_size)
    }
}

// impl Measured3d for Cuboid {
//     /// Get the surface area of the cuboid
//     #[inline(always)]
//     fn area(&self) -> f32 {
//         8.0 * (self.half_size.x * self.half_size.y
//             + self.half_size.y * self.half_size.z
//             + self.half_size.x * self.half_size.z)
//     }

//     /// Get the volume of the cuboid
//     #[inline(always)]
//     fn volume(&self) -> f32 {
//         8.0 * self.half_size.x * self.half_size.y * self.half_size.z
//     }
// }
