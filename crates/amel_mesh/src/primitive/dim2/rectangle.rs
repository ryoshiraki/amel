// use super::super::super::indices::*;
use crate::mesh::Mesh;
use crate::primitive::Meshable;
use amel_gpu::prelude::*;
use amel_math::primitive::dim2::Rectangle;

#[derive(Clone, Copy, Debug, Default)]
pub struct RectangleMeshBuilder {
    pub rectangle: Rectangle,
    pub wireframe: bool,
}

impl RectangleMeshBuilder {
    #[inline]
    pub fn new(width: f32, height: f32, wireframe: bool) -> Self {
        Self {
            rectangle: Rectangle::new(width, height),
            wireframe,
        }
    }

    #[inline]
    pub const fn wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    pub fn build(&self) -> Mesh {
        let positions = vec![
            [0.0, 0.0, 0.0],
            [self.rectangle.width(), 0.0, 0.0],
            [self.rectangle.width(), self.rectangle.height(), 0.0],
            [0.0, self.rectangle.height(), 0.0],
        ];
        let normals = vec![[0.0, 0.0, 1.0]; 4];
        let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

        let indices: Vec<u16> = if self.wireframe {
            vec![0, 1, 2, 3, 0]
        } else {
            vec![0, 1, 2, 2, 3, 0]
        };

        let primitive_topology = if self.wireframe {
            wgpu::PrimitiveTopology::LineStrip
        } else {
            wgpu::PrimitiveTopology::TriangleList
        };

        Mesh::new(primitive_topology)
            .with_inserted_indices(indices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}

impl Meshable for Rectangle {
    type Output = RectangleMeshBuilder;

    fn to_mesh_builder(&self) -> Self::Output {
        RectangleMeshBuilder {
            rectangle: *self,
            ..Default::default()
        }
    }
}

impl From<Rectangle> for Mesh {
    fn from(rectangle: Rectangle) -> Self {
        rectangle.to_mesh_builder().build()
    }
}

impl From<RectangleMeshBuilder> for Mesh {
    fn from(rectangle: RectangleMeshBuilder) -> Self {
        rectangle.build()
    }
}
