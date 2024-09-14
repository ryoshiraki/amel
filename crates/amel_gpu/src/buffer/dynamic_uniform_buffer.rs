use bytemuck::Pod;
use std::num::NonZeroU64;
use std::ops::Deref;

use super::Buffer;

#[derive(Debug)]
pub struct DynamicUniformBuffer {
    buffer: Buffer,
    item_size: usize,
    // alignment: usize,
    aligned_item_size: usize,
}

impl DynamicUniformBuffer {
    pub fn new<T: Pod>(
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        item_size: usize,
        item_count: usize,
    ) -> Self {
        let alignment = device.limits().min_uniform_buffer_offset_alignment as usize;
        let aligned_item_size = ((item_size + alignment - 1) / alignment) * alignment;
        let buffer = Buffer::new(device, usage, item_size, item_count * aligned_item_size);

        Self {
            buffer,
            item_size,
            // alignment,
            aligned_item_size,
        }
    }

    pub fn new_with_data<T: Pod>(
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        item_size: usize,
        bytes: &[u8],
    ) -> Self {
        let alignment = device.limits().min_uniform_buffer_offset_alignment as usize;
        let aligned_item_size = ((item_size + alignment - 1) / alignment) * alignment;
        let buffer = Buffer::new_with_data(device, usage, item_size, bytes);

        Self {
            buffer,
            item_size,
            // alignment,
            aligned_item_size,
        }
    }

    pub fn update<T: Pod>(&self, queue: &wgpu::Queue, index: usize, data: &T) {
        assert!(
            index * self.aligned_item_size < self.buffer.size(),
            "Index out of bounds"
        );
        let offset = (index * self.aligned_item_size) as u64;
        let bytes = bytemuck::bytes_of(data);
        queue.write_buffer(&self.buffer, offset, bytes);
    }

    pub fn binding(&self, index: usize) -> wgpu::BufferBinding {
        assert!(
            index * self.aligned_item_size < self.buffer.size(),
            "Index out of bounds"
        );
        let offset = (index * self.aligned_item_size) as u64;

        wgpu::BufferBinding {
            buffer: &self.buffer,
            offset,
            size: NonZeroU64::new(self.item_size as u64),
        }
    }
}

impl Deref for DynamicUniformBuffer {
    type Target = Buffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
