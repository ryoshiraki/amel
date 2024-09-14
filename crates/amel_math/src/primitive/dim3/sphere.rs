use super::Primitive3d;
use crate::prelude::*;

/// A sphere primitive
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Sphere {
    /// The radius of the sphere
    pub radius: f32,
}
impl Primitive3d for Sphere {}

impl Default for Sphere {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

impl Sphere {
    #[inline(always)]
    pub const fn new(radius: f32) -> Self {
        Self { radius }
    }

    #[inline(always)]
    pub fn diameter(&self) -> f32 {
        2.0 * self.radius
    }

    #[inline(always)]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let distance_squared = point.length_squared();

        if distance_squared <= self.radius.powi(2) {
            // The point is inside the sphere.
            point
        } else {
            // The point is outside the sphere.
            // Find the closest point on the surface of the sphere.
            let dir_to_point = point / distance_squared.sqrt();
            self.radius * dir_to_point
        }
    }
}

// impl Measured3d for Sphere {
//     /// Get the surface area of the sphere
//     #[inline(always)]
//     fn area(&self) -> f32 {
//         4.0 * PI * self.radius.powi(2)
//     }

//     /// Get the volume of the sphere
//     #[inline(always)]
//     fn volume(&self) -> f32 {
//         4.0 * FRAC_PI_3 * self.radius.powi(3)
//     }
// }
