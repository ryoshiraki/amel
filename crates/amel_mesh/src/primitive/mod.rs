pub mod dim2;
pub mod dim3;

pub use dim2::*;
pub use dim3::*;

pub trait Meshable {
    type Output;
    fn to_mesh_builder(&self) -> Self::Output;
}
