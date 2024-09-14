use crate::mesh::Mesh;
use crate::primitive::Meshable;

use amel_math::primitive::dim2::{Ellipse, RegularPolygon};

#[derive(Clone, Copy, Debug, Default)]
pub struct RegularPolygonMeshBuilder {
    pub regular_polygon: RegularPolygon,
    pub wireframe: bool,
}

impl RegularPolygonMeshBuilder {
    #[inline]
    pub fn new(circumradius: f32, sides: usize, wireframe: bool) -> Self {
        Self {
            regular_polygon: RegularPolygon::new(circumradius, sides),
            wireframe,
        }
    }

    #[inline]
    pub const fn wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    pub fn build(&self) -> Mesh {
        Ellipse::new(
            self.regular_polygon.circumcircle.radius,
            self.regular_polygon.circumcircle.radius,
        )
        .to_mesh_builder()
        .resolution(self.regular_polygon.sides)
        .wireframe(self.wireframe)
        .build()
    }
}

impl Meshable for RegularPolygon {
    type Output = RegularPolygonMeshBuilder;

    fn to_mesh_builder(&self) -> Self::Output {
        RegularPolygonMeshBuilder {
            regular_polygon: *self,
            ..Default::default()
        }
    }
}

impl From<RegularPolygon> for Mesh {
    fn from(regular_polygon: RegularPolygon) -> Self {
        regular_polygon.to_mesh_builder().build()
    }
}

impl From<RegularPolygonMeshBuilder> for Mesh {
    fn from(regular_polygon: RegularPolygonMeshBuilder) -> Self {
        regular_polygon.build()
    }
}
