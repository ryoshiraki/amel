use crate::mesh::Mesh;
use crate::primitive::Meshable;
use amel_gpu::prelude::*;
use amel_math::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Triangle2dMeshBuilder {
    pub triangle: Triangle2d,
    pub wireframe: bool,
}

impl Triangle2dMeshBuilder {
    #[inline]
    pub const fn new(a: Vec2, b: Vec2, c: Vec2, wireframe: bool) -> Self {
        Self {
            triangle: Triangle2d {
                vertices: [a, b, c],
            },
            wireframe,
        }
    }

    #[inline]
    pub const fn wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    pub fn build(&self) -> Mesh {
        let [a, b, c] = self.triangle.vertices;

        let positions = vec![[a.x, a.y, 0.0], [b.x, b.y, 0.0], [c.x, c.y, 0.0]];
        let normals = vec![[0.0, 0.0, 1.0]; 3];
        // let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];

        // The extents of the bounding box of the triangle,
        // used to compute the UV coordinates of the points.
        let extents = a.min(b).min(c).abs().max(a.max(b).max(c)) * Vec2::new(1.0, -1.0);

        let uvs = vec![
            a / extents / 2.0 + 0.5,
            b / extents / 2.0 + 0.5,
            c / extents / 2.0 + 0.5,
        ];

        let indices: Vec<u16> = if self.wireframe {
            vec![0, 1, 2, 0]
        } else {
            vec![0, 1, 2]
        };

        let primitive_topology = if self.wireframe {
            wgpu::PrimitiveTopology::LineStrip
        } else {
            wgpu::PrimitiveTopology::TriangleList
        };

        Mesh::new(primitive_topology)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_indices(indices)
    }
}

impl Meshable for Triangle2d {
    type Output = Triangle2dMeshBuilder;

    fn to_mesh_builder(&self) -> Self::Output {
        Triangle2dMeshBuilder {
            triangle: *self,
            ..Default::default()
        }
    }
}

impl From<Triangle2d> for Mesh {
    fn from(triangle: Triangle2d) -> Self {
        triangle.to_mesh_builder().build()
    }
}

impl From<Triangle2dMeshBuilder> for Mesh {
    fn from(triangle: Triangle2dMeshBuilder) -> Self {
        triangle.build()
    }
}
