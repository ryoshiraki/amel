use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub radius: f32,
}
impl Primitive2d for Circle {}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

impl Circle {
    #[inline(always)]
    pub const fn new(radius: f32) -> Self {
        Self { radius }
    }

    #[inline(always)]
    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    #[inline(always)]
    pub fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    #[inline(always)]
    pub fn perimeter(&self) -> f32 {
        PI * self.radius * 2.0
    }
}
