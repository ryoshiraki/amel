use crate::mesh::Mesh;
use crate::primitive::Meshable;

use amel_gpu::prelude::*;
use amel_math::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Polyline2dMeshBuilder<const N: usize> {
    pub polyline: Polyline2d<N>,
    pub closed: bool,
}

impl<const N: usize> Polyline2dMeshBuilder<N> {
    #[inline]
    pub fn new(vertices: impl IntoIterator<Item = Vec2>) -> Self {
        Self {
            polyline: Polyline2d::from_iter(vertices),
            closed: false,
        }
    }

    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    pub fn build(&self) -> Mesh {
        let positions: Vec<[f32; 3]> = self
            .polyline
            .vertices
            .iter()
            .map(|v| [v.x, v.y, 0.0])
            .collect();
        let normals = vec![[0.0, 0.0, 1.0]; N];
        let uvs = vec![[0.0, 0.0]; N];

        let mut indices: Vec<u16> =
            Vec::with_capacity(if self.closed { (N * 3) + 1 } else { N * 3 });

        for i in 0..N {
            indices.push(i as u16);
            if self.closed && i == N - 1 {
                indices.push(0);
            }
        }

        if !self.closed {
            indices.pop();
        }

        Mesh::new(wgpu::PrimitiveTopology::LineStrip)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_indices(indices)
    }
}

impl<const N: usize> Meshable for Polyline2d<N> {
    type Output = Polyline2dMeshBuilder<N>;

    fn to_mesh_builder(&self) -> Self::Output {
        Polyline2dMeshBuilder {
            polyline: *self,
            closed: false,
        }
    }
}

impl<const N: usize> From<Polyline2d<N>> for Mesh {
    fn from(polyline: Polyline2d<N>) -> Self {
        polyline.to_mesh_builder().build()
    }
}

impl<const N: usize> From<Polyline2dMeshBuilder<N>> for Mesh {
    fn from(polyline: Polyline2dMeshBuilder<N>) -> Self {
        polyline.build()
    }
}
