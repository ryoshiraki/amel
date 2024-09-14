use super::Circle;
use super::Primitive2d;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularPolygon {
    pub circumcircle: Circle,
    pub sides: usize,
}
impl Primitive2d for RegularPolygon {}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self {
            circumcircle: Circle::default(),
            sides: 6,
        }
    }
}

impl RegularPolygon {
    #[inline(always)]
    pub fn new(circumradius: f32, sides: usize) -> Self {
        assert!(circumradius > 0.0, "polygon has a non-positive radius");
        assert!(sides > 2, "polygon has less than 3 sides");
        Self {
            circumcircle: Circle {
                radius: circumradius,
            },
            sides,
        }
    }

    #[inline(always)]
    pub fn circumradius(&self) -> f32 {
        self.circumcircle.radius
    }

    #[inline(always)]
    pub fn inradius(&self) -> f32 {
        self.circumradius() * (PI / self.sides as f32).cos()
    }

    #[inline(always)]
    pub fn side_length(&self) -> f32 {
        2.0 * self.circumradius() * (PI / self.sides as f32).sin()
    }

    #[inline(always)]
    pub fn area(&self) -> f32 {
        let angle: f32 = 2.0 * PI / (self.sides as f32);
        (self.sides as f32) * self.circumradius().powi(2) * angle.sin() / 2.0
    }

    #[inline(always)]
    pub fn perimeter(&self) -> f32 {
        self.sides as f32 * self.side_length()
    }

    #[inline(always)]
    pub fn internal_angle_degrees(&self) -> f32 {
        (self.sides - 2) as f32 / self.sides as f32 * 180.0
    }

    #[inline(always)]
    pub fn internal_angle_radians(&self) -> f32 {
        (self.sides - 2) as f32 * PI / self.sides as f32
    }

    #[inline(always)]
    pub fn external_angle_degrees(&self) -> f32 {
        360.0 / self.sides as f32
    }

    #[inline(always)]
    pub fn external_angle_radians(&self) -> f32 {
        2.0 * PI / self.sides as f32
    }

    pub fn vertices(self, rotation: f32) -> impl IntoIterator<Item = Vec2> {
        // Add pi/2 so that the polygon has a vertex at the top (sin is 1.0 and cos is 0.0)
        let start_angle = rotation + std::f32::consts::FRAC_PI_2;
        let step = std::f32::consts::TAU / self.sides as f32;

        (0..self.sides).map(move |i| {
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = theta.sin_cos();
            Vec2::new(cos, sin) * self.circumcircle.radius
        })
    }
}
