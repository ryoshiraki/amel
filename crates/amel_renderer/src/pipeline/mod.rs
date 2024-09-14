use amel_math::prelude::*;
use bytemuck::{Pod, Zeroable};

pub mod default;

pub mod prelude {
    pub use super::default::*;
}

#[repr(C)]
#[derive(Default, Copy, Clone, Pod, Zeroable)]
/// The uniforms for the shader.
pub struct Uniforms {
    pub ortho: [f32; 16],
    pub transform: [f32; 16],
    pub color: [f32; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            ortho: Mat4::IDENTITY.to_cols_array(),
            transform: Mat4::IDENTITY.to_cols_array(),
            color: Vec4::ONE.to_array(),
        }
    }

    pub fn set_color(mut self, color: Vec4) -> Self {
        self.color = color.to_array();
        self
    }

    pub fn set_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform.to_cols_array();
        self
    }

    pub fn set_ortho(mut self, ortho: Mat4) -> Self {
        self.ortho = ortho.to_cols_array();
        self
    }
}
