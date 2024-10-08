use crate::mesh::Mesh;
use crate::primitive::Meshable;
use amel_gpu::prelude::*;
use amel_math::prelude::*;
/// A builder used for creating a [`Mesh`] with a [`Cuboid`] shape.
pub struct CuboidMeshBuilder {
    half_size: Vec3,
    wireframe: bool,
}

impl CuboidMeshBuilder {
    pub fn new(half_size: Vec3) -> Self {
        Self {
            half_size,
            wireframe: false,
        }
    }

    pub fn wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    pub fn build(&self) -> Mesh {
        let min = -self.half_size;
        let max = self.half_size;

        // Suppose Y-up right hand, and camera look from +Z to -Z
        let vertices = &[
            // Front
            ([min.x, min.y, max.z], [0.0, 0.0, 1.0], [0.0, 0.0]),
            ([max.x, min.y, max.z], [0.0, 0.0, 1.0], [1.0, 0.0]),
            ([max.x, max.y, max.z], [0.0, 0.0, 1.0], [1.0, 1.0]),
            ([min.x, max.y, max.z], [0.0, 0.0, 1.0], [0.0, 1.0]),
            // Back
            ([min.x, max.y, min.z], [0.0, 0.0, -1.0], [1.0, 0.0]),
            ([max.x, max.y, min.z], [0.0, 0.0, -1.0], [0.0, 0.0]),
            ([max.x, min.y, min.z], [0.0, 0.0, -1.0], [0.0, 1.0]),
            ([min.x, min.y, min.z], [0.0, 0.0, -1.0], [1.0, 1.0]),
            // Right
            ([max.x, min.y, min.z], [1.0, 0.0, 0.0], [0.0, 0.0]),
            ([max.x, max.y, min.z], [1.0, 0.0, 0.0], [1.0, 0.0]),
            ([max.x, max.y, max.z], [1.0, 0.0, 0.0], [1.0, 1.0]),
            ([max.x, min.y, max.z], [1.0, 0.0, 0.0], [0.0, 1.0]),
            // Left
            ([min.x, min.y, max.z], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            ([min.x, max.y, max.z], [-1.0, 0.0, 0.0], [0.0, 0.0]),
            ([min.x, max.y, min.z], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            ([min.x, min.y, min.z], [-1.0, 0.0, 0.0], [1.0, 1.0]),
            // Top
            ([max.x, max.y, min.z], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([min.x, max.y, min.z], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([min.x, max.y, max.z], [0.0, 1.0, 0.0], [0.0, 1.0]),
            ([max.x, max.y, max.z], [0.0, 1.0, 0.0], [1.0, 1.0]),
            // Bottom
            ([max.x, min.y, max.z], [0.0, -1.0, 0.0], [0.0, 0.0]),
            ([min.x, min.y, max.z], [0.0, -1.0, 0.0], [1.0, 0.0]),
            ([min.x, min.y, min.z], [0.0, -1.0, 0.0], [1.0, 1.0]),
            ([max.x, min.y, min.z], [0.0, -1.0, 0.0], [0.0, 1.0]),
        ];

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let indices: Vec<u16> = if self.wireframe {
            vec![
                0, 1, 1, 2, 2, 3, 3, 0, // front
                4, 5, 5, 6, 6, 7, 7, 4, // back
                8, 9, 9, 10, 10, 11, 11, 8, // right
                12, 13, 13, 14, 14, 15, 15, 12, // left
                16, 17, 17, 18, 18, 19, 19, 16, // top
                20, 21, 21, 22, 22, 23, 23, 20, // bottom
            ]
        } else {
            vec![
                0, 1, 2, 2, 3, 0, // front
                4, 5, 6, 6, 7, 4, // back
                8, 9, 10, 10, 11, 8, // right
                12, 13, 14, 14, 15, 12, // left
                16, 17, 18, 18, 19, 16, // top
                20, 21, 22, 22, 23, 20, // bottom
            ]
        };

        let primitive_topology = if self.wireframe {
            wgpu::PrimitiveTopology::LineList
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

impl Meshable for Cuboid {
    type Output = CuboidMeshBuilder;

    fn to_mesh_builder(&self) -> Self::Output {
        CuboidMeshBuilder::new(self.half_size)
    }
}

impl From<Cuboid> for Mesh {
    fn from(cuboid: Cuboid) -> Self {
        cuboid.to_mesh_builder().build()
    }
}
