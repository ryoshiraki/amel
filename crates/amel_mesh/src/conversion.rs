use super::attribute::VertexAttributeValues;
use amel_math::prelude::*;

#[derive(Debug, Clone)]
pub struct FromVertexAttributeError {
    from: VertexAttributeValues,
    into: &'static str,
}

impl FromVertexAttributeError {
    fn new<T: 'static>(from: VertexAttributeValues) -> Self {
        Self {
            from,
            into: std::any::type_name::<T>(),
        }
    }
}

macro_rules! impl_from {
    ($from:ty, $variant:tt) => {
        impl From<Vec<$from>> for VertexAttributeValues {
            fn from(vec: Vec<$from>) -> Self {
                VertexAttributeValues::$variant(vec)
            }
        }
    };
}

macro_rules! impl_from_into {
    ($from:ty, $variant:tt) => {
        impl From<Vec<$from>> for VertexAttributeValues {
            fn from(vec: Vec<$from>) -> Self {
                let vec: Vec<_> = vec.into_iter().map(|t| t.into()).collect();
                VertexAttributeValues::$variant(vec)
            }
        }
    };
}

impl_from!(f32, Float32);
impl_from!([f32; 2], Float32x2);
impl_from_into!(Vec2, Float32x2);
impl_from!([f32; 3], Float32x3);
impl_from_into!(Vec3, Float32x3);
// impl_from_into!(Vec3A, Float32x3);
impl_from!([f32; 4], Float32x4);
// impl_from_into!(Vec4, Float32x4);

impl_from!(i32, Sint32);
impl_from!([i32; 2], Sint32x2);
impl_from_into!(IVec2, Sint32x2);
impl_from!([i32; 3], Sint32x3);
impl_from_into!(IVec3, Sint32x3);
impl_from!([i32; 4], Sint32x4);
// impl_from_into!(IVec4, Sint32x4);

impl_from!(u32, Uint32);
impl_from!([u32; 2], Uint32x2);
impl_from_into!(UVec2, Uint32x2);
impl_from!([u32; 3], Uint32x3);
impl_from_into!(UVec3, Uint32x3);
impl_from!([u32; 4], Uint32x4);
// impl_from_into!(UVec4, Uint32x4);

macro_rules! impl_try_from {
    ($into:ty, $($variant:tt), +) => {
        impl TryFrom<VertexAttributeValues> for Vec<$into> {
            type Error = FromVertexAttributeError;

            fn try_from(value: VertexAttributeValues) -> Result<Self, Self::Error> {
                match value {
                    $(VertexAttributeValues::$variant(value)) |+ => Ok(value),
                    _ => Err(FromVertexAttributeError::new::<Self>(value)),
                }
            }
        }
    };
}

macro_rules! impl_try_from_into {
    ($into:ty, $($variant:tt), +) => {
        impl TryFrom<VertexAttributeValues> for Vec<$into> {
            type Error = FromVertexAttributeError;

            fn try_from(value: VertexAttributeValues) -> Result<Self, Self::Error> {
                match value {
                    $(VertexAttributeValues::$variant(value)) |+ => {
                        Ok(value.into_iter().map(|t| t.into()).collect())
                    }
                    _ => Err(FromVertexAttributeError::new::<Self>(value)),
                }
            }
        }
    };
}

impl_try_from!(f32, Float32);
impl_try_from!([f32; 2], Float32x2);
impl_try_from_into!(Vec2, Float32x2);
impl_try_from!([f32; 3], Float32x3);
impl_try_from_into!(Vec3, Float32x3);
// impl_try_from_into!(Vec3A, Float32x3);
impl_try_from!([f32; 4], Float32x4);
// impl_try_from_into!(Vec4, Float32x4);

impl_try_from!(i32, Sint32);
impl_try_from!([i32; 2], Sint32x2);
impl_try_from_into!(IVec2, Sint32x2);
impl_try_from!([i32; 3], Sint32x3);
impl_try_from_into!(IVec3, Sint32x3);
impl_try_from!([i32; 4], Sint32x4);
// impl_try_from_into!(IVec4, Sint32x4);

impl_try_from!(u32, Uint32);
impl_try_from!([u32; 2], Uint32x2);
impl_try_from_into!(UVec2, Uint32x2);
impl_try_from!([u32; 3], Uint32x3);
impl_try_from_into!(UVec3, Uint32x3);
impl_try_from!([u32; 4], Uint32x4);
// impl_try_from_into!(UVec4, Uint32x4);

impl_try_from!([i8; 2], Sint8x2, Snorm8x2);
impl_try_from!([i8; 4], Sint8x4, Snorm8x4);

impl_try_from!([u8; 2], Uint8x2, Unorm8x2);
impl_try_from!([u8; 4], Uint8x4, Unorm8x4);

impl_try_from!([i16; 2], Sint16x2, Snorm16x2);
impl_try_from!([i16; 4], Sint16x4, Snorm16x4);

impl_try_from!([u16; 2], Uint16x2, Unorm16x2);
impl_try_from!([u16; 4], Uint16x4, Unorm16x4);
