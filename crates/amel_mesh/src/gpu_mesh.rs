use std::collections::HashMap;

use crate::prelude::Indices;

use super::mesh::Mesh;
use amel_gpu::prelude::*;

// #[derive(Debug)]
// pub struct GpuMesh {
//     pub vertex_buffer: Buffer,
//     pub index_buffer: Option<Buffer>,
//     pub primitive_topology: gpu::wgpu::PrimitiveTopology,
//     pub attributes: VertexAttributes,
//     // pub layout: wgpu::VertexBufferLayout<'a>,
// }

#[derive(Debug)]
pub struct GpuMesh {
    pub primitive_topology: wgpu::PrimitiveTopology,
    pub index_format: wgpu::IndexFormat,
    pub vertex_buffers: HashMap<u32, Buffer>,
    pub index_buffer: Option<Buffer>,
}

impl GpuMesh {
    pub fn buffer(&self, location: u32) -> Option<&Buffer> {
        self.vertex_buffers.get(&location)
    }

    pub fn buffer_mut(&mut self, location: u32) -> Option<&mut Buffer> {
        self.vertex_buffers.get_mut(&location)
    }

    pub fn index_buffer(&self) -> Option<&Buffer> {
        self.index_buffer.as_ref()
    }

    pub fn index_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.index_buffer.as_mut()
    }

    pub fn draw<'a, E: wgpu::util::RenderEncoder<'a>>(&'a self, encoder: &mut E) {
        for (index, buffer) in &self.vertex_buffers {
            encoder.set_vertex_buffer(*index, buffer.slice());
        }
        // encoder.set_vertex_buffer(0, self.vertex_buffer.slice());

        match &self.index_buffer {
            Some(index_buffer) => {
                encoder.set_index_buffer(index_buffer.slice(), wgpu::IndexFormat::Uint16);
                encoder.draw_indexed(0..index_buffer.count() as u32, 0, 0..1);
            }
            None => {
                encoder.draw(
                    0..self.vertex_buffers.values().next().unwrap().count() as u32,
                    0..1,
                );
                // encoder.draw(0..self.vertex_buffer.count() as u32, 0..1);
            }
        }
    }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a GpuMesh);
    fn draw_mesh_ranged(&mut self, mesh: &'a GpuMesh, vertex_range: std::ops::Range<u32>);
}

impl<'a> DrawMesh<'a> for wgpu::RenderPass<'a> {
    #[inline]
    fn draw_mesh_ranged(&mut self, mesh: &'a GpuMesh, vertex_range: std::ops::Range<u32>) {
        for (index, buffer) in &mesh.vertex_buffers {
            self.set_vertex_buffer(*index, buffer.slice());
        }

        if let Some(index_buffer) = &mesh.index_buffer {
            self.set_index_buffer(index_buffer.slice(), wgpu::IndexFormat::Uint16);
            self.draw_indexed(vertex_range, 0, 0..1);
        } else if mesh.vertex_buffers.contains_key(&0) {
            self.draw(vertex_range, 0..1);
        }
    }

    fn draw_mesh(&mut self, mesh: &'a GpuMesh) {
        let vertex_count = if let Some(index_buffer) = &mesh.index_buffer {
            index_buffer.count() as u32
        } else if let Some(buf) = mesh.vertex_buffers.get(&0) {
            buf.count() as u32
        } else {
            0
        };

        self.draw_mesh_ranged(mesh, 0..vertex_count);
    }
}

impl<'a> DrawMesh<'a> for wgpu::RenderBundleEncoder<'a> {
    #[inline]
    fn draw_mesh_ranged(&mut self, mesh: &'a GpuMesh, vertex_range: std::ops::Range<u32>) {
        for (index, buffer) in &mesh.vertex_buffers {
            self.set_vertex_buffer(*index, buffer.slice());
        }

        if let Some(index_buffer) = &mesh.index_buffer {
            self.set_index_buffer(index_buffer.slice(), mesh.index_format);
            self.draw_indexed(vertex_range, 0, 0..1);
        } else if mesh.vertex_buffers.contains_key(&0) {
            self.draw(vertex_range, 0..1);
        }
    }

    fn draw_mesh(&mut self, mesh: &'a GpuMesh) {
        let vertex_count = if let Some(index_buffer) = &mesh.index_buffer {
            index_buffer.count() as u32
        } else if let Some(buf) = mesh.vertex_buffers.get(&0) {
            buf.count() as u32
        } else {
            0
        };

        self.draw_mesh_ranged(mesh, 0..vertex_count);
    }
}

pub trait ToGpuMesh {
    fn to_gpu(&self, device: &wgpu::Device) -> GpuMesh;
}

impl ToGpuMesh for Mesh {
    fn to_gpu(&self, device: &wgpu::Device) -> GpuMesh {
        let mut vertex_buffers = HashMap::new();

        for (location, data) in self.attributes.iter() {
            let vertex_buffer = BufferBuilder::new().vertex().copy_dst().build_with_data(
                device,
                data.item_size() as usize,
                data.as_bytes(),
            );

            vertex_buffers.insert(*location, vertex_buffer);
        }

        let index_buffer = self.indices().map(|indices| {
            BufferBuilder::new().index().build_with_data(
                device,
                indices.item_size(),
                indices.as_bytes(),
            )
        });

        let index_format = match self.indices() {
            Some(Indices::U16(_)) => wgpu::IndexFormat::Uint16,
            Some(Indices::U32(_)) => wgpu::IndexFormat::Uint32,
            None => wgpu::IndexFormat::Uint16,
        };
        GpuMesh {
            primitive_topology: self.primitive_topology,
            vertex_buffers,
            index_buffer,
            index_format,
        }
    }

    // fn to_gpu(&self, device: &wgpu::Device) -> GpuMesh {
    //     let vertex_buffer_data = self.get_vertex_buffer_data();

    //     let vertex_buffer = BufferBuilder::new()
    //         .vertex()
    //         .copy_dst()
    //         .build_with_data(device, &vertex_buffer_data);

    //     let index_buffer = self.indices().map(|indices| match indices {
    //         Indices::U16(data) => BufferBuilder::new().index().build_with_data(device, data),
    //         Indices::U32(data) => BufferBuilder::new().index().build_with_data(device, data),
    //     });

    //     let mut attributes = VertexAttributes::new();
    //     for data in self.attributes.values() {
    //         attributes = attributes.add_attribute(data.attribute.id, data.attribute.format);
    //     }

    //     GpuMesh {
    //         vertex_buffer,
    //         index_buffer,
    //         primitive_topology: self.primitive_topology(),
    //         attributes,
    //     }
    // }
}
