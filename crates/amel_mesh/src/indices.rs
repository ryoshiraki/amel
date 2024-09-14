use bytemuck::Pod;
use amel_gpu::prelude::*;

#[derive(Debug, Clone)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    pub fn as_bytes<T: Pod>(&self) -> &[T] {
        match self {
            Indices::U16(data) => bytemuck::cast_slice(data),
            Indices::U32(data) => bytemuck::cast_slice(data),
        }
    }

    pub fn index_format(&self) -> wgpu::IndexFormat {
        match self {
            Indices::U16(_) => wgpu::IndexFormat::Uint16,
            Indices::U32(_) => wgpu::IndexFormat::Uint32,
        }
    }

    pub fn item_size(&self) -> usize {
        match self {
            Indices::U16(_) => std::mem::size_of::<u16>(),
            Indices::U32(_) => std::mem::size_of::<u32>(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Indices::U16(data) => data.len(),
            Indices::U32(data) => data.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Indices::U16(data) => data.is_empty(),
            Indices::U32(data) => data.is_empty(),
        }
    }
}

impl From<Vec<u16>> for Indices {
    fn from(data: Vec<u16>) -> Self {
        Indices::U16(data)
    }
}

impl From<Vec<u32>> for Indices {
    fn from(data: Vec<u32>) -> Self {
        Indices::U32(data)
    }
}
