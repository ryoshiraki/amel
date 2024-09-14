use crate::mesh::Mesh;
use crate::primitive::Meshable;

use amel_gpu::prelude::*;
use amel_math::primitive::dim2::Ellipse;

#[derive(Clone, Copy, Debug)]
pub struct EllipseMeshBuilder {
    pub ellipse: Ellipse,
    pub resolution: usize,
    pub wireframe: bool,
}

impl Default for EllipseMeshBuilder {
    fn default() -> Self {
        Self {
            ellipse: Ellipse::default(),
            resolution: 32,
            wireframe: false,
        }
    }
}

impl EllipseMeshBuilder {
    #[inline]
    pub const fn new(
        half_width: f32,
        half_height: f32,
        resolution: usize,
        wireframe: bool,
    ) -> Self {
        Self {
            ellipse: Ellipse::new(half_width, half_height),
            resolution,
            wireframe,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    #[inline]
    pub const fn wireframe(mut self, wireframe: bool) -> Self {
        self.wireframe = wireframe;
        self
    }

    pub fn build(&self) -> Mesh {
        let mut positions = Vec::with_capacity(self.resolution);
        let normals = vec![[0.0, 0.0, 1.0]; self.resolution];
        let mut uvs = Vec::with_capacity(self.resolution);

        // Add pi/2 so that there is a vertex at the top (sin is 1.0 and cos is 0.0)
        let start_angle = std::f32::consts::FRAC_PI_2;
        let step = std::f32::consts::TAU / self.resolution as f32;

        for i in 0..self.resolution {
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = theta.sin_cos();
            let x = cos * self.ellipse.half_size.x;
            let y = sin * self.ellipse.half_size.y;

            positions.push([x, y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }

        let mut indices: Vec<u16> = if self.wireframe {
            Vec::with_capacity(self.resolution)
        } else {
            Vec::with_capacity((self.resolution - 2) * 3)
        };

        if self.wireframe {
            for i in 0..self.resolution {
                indices.push(i as u16);
            }
            indices.push(0);
        } else {
            for i in 1..(self.resolution - 1) {
                indices.extend_from_slice(&[0, i as u16, (i + 1) as u16]);
            }
        }

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

impl Meshable for Ellipse {
    type Output = EllipseMeshBuilder;
    fn to_mesh_builder(&self) -> Self::Output {
        EllipseMeshBuilder {
            ellipse: *self,
            ..Default::default()
        }
    }
}

impl From<Ellipse> for Mesh {
    fn from(ellipse: Ellipse) -> Self {
        ellipse.to_mesh_builder().build()
    }
}

impl From<EllipseMeshBuilder> for Mesh {
    fn from(ellipse: EllipseMeshBuilder) -> Self {
        ellipse.build()
    }
}
