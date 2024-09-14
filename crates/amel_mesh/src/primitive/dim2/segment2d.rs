// use super::super::super::indices::*;
use crate::mesh::Mesh;
use crate::primitive::Meshable;
use amel_math::prelude::*;

use super::polyline2d::Polyline2dMeshBuilder;

impl Meshable for Segment2d {
    type Output = Polyline2dMeshBuilder<2>;
    fn to_mesh_builder(&self) -> Self::Output {
        Polyline2dMeshBuilder {
            polyline: Polyline2d::new([self.a(), self.b()]),
            closed: false,
        }
    }
}

impl From<Segment2d> for Mesh {
    fn from(segment: Segment2d) -> Self {
        segment.to_mesh_builder().build()
    }
}
