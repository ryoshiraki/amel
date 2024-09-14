use crate::mesh::Mesh;
use crate::primitive::Meshable;

use super::EllipseMeshBuilder;
use amel_math::primitive::dim2::{Circle, Ellipse};

impl Meshable for Circle {
    type Output = EllipseMeshBuilder;
    fn to_mesh_builder(&self) -> Self::Output {
        EllipseMeshBuilder {
            ellipse: Ellipse::new(self.radius, self.radius),
            ..Default::default()
        }
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Self {
        circle.to_mesh_builder().build()
    }
}
