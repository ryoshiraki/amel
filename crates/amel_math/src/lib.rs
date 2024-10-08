pub mod matrix_stack;
pub mod primitive;
// pub mod random;

pub mod prelude {

    pub use super::matrix_stack::*;
    pub use super::primitive::prelude::*;
    // pub use super::random::*;

    pub use std::f32::consts::*;

    pub use glam::{
        BVec2, BVec3, BVec4, EulerRot, FloatExt, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
        UVec2, UVec3, UVec4, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles,
    };
}
