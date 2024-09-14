use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment2d {
    pub direction: Vec2,
    pub half_length: f32,
}

impl Primitive2d for Segment2d {}

impl Segment2d {
    #[inline(always)]
    pub fn new(direction: Vec2, length: f32) -> Self {
        Self {
            direction,
            half_length: length * 0.5,
        }
    }

    #[inline(always)]
    pub fn from_points(a: Vec2, b: Vec2) -> Self {
        Self::new(b - a, 0.5 * (b - a).length())
    }

    #[inline(always)]
    pub fn a(&self) -> Vec2 {
        -self.direction * self.half_length
    }

    #[inline(always)]
    pub fn b(&self) -> Vec2 {
        self.direction * self.half_length
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        2.0 * self.half_length
    }

    #[inline(always)]
    pub fn area(&self) -> f32 {
        0.0
    }
}
