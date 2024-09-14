pub mod buffer_builder;
pub use buffer_builder::*;
pub use wgpu::util::DeviceExt;

pub mod dynamic_uniform_buffer;
pub use dynamic_uniform_buffer::*;

use std::ops::Deref;
// use std::{num::NonZeroU64, ops::Deref};

#[derive(Debug)]
pub struct Buffer {
    buffer: wgpu::Buffer,
    item_size: usize,
    item_count: usize,
    usage: wgpu::BufferUsages,
}

impl Buffer {
    pub fn new(
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        item_size: usize,
        item_count: usize,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (item_size * item_count) as u64,
            usage,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            item_size,
            item_count,
            usage,
        }
    }

    pub fn new_with_data(
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        item_size: usize,
        bytes: &[u8],
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage,
        });

        let item_count = bytes.len() / item_size;

        Self {
            buffer,
            item_size,
            item_count,
            usage,
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.item_count
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.item_size * self.item_count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    #[inline]
    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }

    // pub fn update<T: Pod>(&self, queue: &wgpu::Queue, data: &T) {
    //     let bytes = bytemuck::bytes_of(data);
    //     queue.write_buffer(&self.buffer, 0, bytes);
    // }

    pub fn update(&self, queue: &wgpu::Queue, offset: u64, bytes: &[u8]) {
        queue.write_buffer(&self.buffer, offset, bytes);
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.slice(..)
    }

    pub fn binding(&self) -> wgpu::BufferBinding {
        wgpu::BufferBinding {
            buffer: &self.buffer,
            offset: 0,
            size: None, //NonZeroU64::new(self.size() as u64),
        }
    }
}

impl Deref for Buffer {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
