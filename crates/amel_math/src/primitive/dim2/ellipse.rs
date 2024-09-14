use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse {
    pub half_size: Vec2,
}
impl Primitive2d for Ellipse {}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            half_size: Vec2::new(0.5, 0.5),
        }
    }
}

impl Ellipse {
    #[inline(always)]
    pub const fn new(half_width: f32, half_height: f32) -> Self {
        Self {
            half_size: Vec2::new(half_width, half_height),
        }
    }

    #[inline(always)]
    pub fn from_size(size: Vec2) -> Self {
        Self {
            half_size: size / 2.0,
        }
    }

    #[inline(always)]
    pub fn semi_major(self) -> f32 {
        self.half_size.max_element()
    }

    #[inline(always)]
    pub fn semi_minor(self) -> f32 {
        self.half_size.min_element()
    }

    #[inline(always)]
    pub fn area(&self) -> f32 {
        PI * self.half_size.x * self.half_size.y
    }
}
