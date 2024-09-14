use bytemuck::cast_slice;
use amel_gpu::prelude::*;

#[derive(Clone, Debug)] //, EnumVariantMeta)]
pub enum VertexAttributeValues {
    Float32(Vec<f32>),
    Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(Vec<[u16; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Snorm8x2(Vec<[i8; 2]>),
    Uint8x2(Vec<[u8; 2]>),
    Unorm8x2(Vec<[u8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint8x4(Vec<[u8; 4]>),
    Unorm8x4(Vec<[u8; 4]>),
}

impl VertexAttributeValues {
    /// Returns the number of vertices in this [`VertexAttributeValues`]. For a single
    /// mesh, all of the [`VertexAttributeValues`] must have the same length.
    #[allow(clippy::match_same_arms)]
    pub fn len(&self) -> usize {
        match self {
            VertexAttributeValues::Float32(values) => values.len(),
            VertexAttributeValues::Sint32(values) => values.len(),
            VertexAttributeValues::Uint32(values) => values.len(),
            VertexAttributeValues::Float32x2(values) => values.len(),
            VertexAttributeValues::Sint32x2(values) => values.len(),
            VertexAttributeValues::Uint32x2(values) => values.len(),
            VertexAttributeValues::Float32x3(values) => values.len(),
            VertexAttributeValues::Sint32x3(values) => values.len(),
            VertexAttributeValues::Uint32x3(values) => values.len(),
            VertexAttributeValues::Float32x4(values) => values.len(),
            VertexAttributeValues::Sint32x4(values) => values.len(),
            VertexAttributeValues::Uint32x4(values) => values.len(),
            VertexAttributeValues::Sint16x2(values) => values.len(),
            VertexAttributeValues::Snorm16x2(values) => values.len(),
            VertexAttributeValues::Uint16x2(values) => values.len(),
            VertexAttributeValues::Unorm16x2(values) => values.len(),
            VertexAttributeValues::Sint16x4(values) => values.len(),
            VertexAttributeValues::Snorm16x4(values) => values.len(),
            VertexAttributeValues::Uint16x4(values) => values.len(),
            VertexAttributeValues::Unorm16x4(values) => values.len(),
            VertexAttributeValues::Sint8x2(values) => values.len(),
            VertexAttributeValues::Snorm8x2(values) => values.len(),
            VertexAttributeValues::Uint8x2(values) => values.len(),
            VertexAttributeValues::Unorm8x2(values) => values.len(),
            VertexAttributeValues::Sint8x4(values) => values.len(),
            VertexAttributeValues::Snorm8x4(values) => values.len(),
            VertexAttributeValues::Uint8x4(values) => values.len(),
            VertexAttributeValues::Unorm8x4(values) => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn item_size(&self) -> u64 {
        let format: wgpu::VertexFormat = self.into();
        format.size()
    }

    // TODO: add vertex format as parameter here and perform type conversions
    /// Flattens the [`VertexAttributeValues`] into a sequence of bytes. This is
    /// useful for serialization and sending to the GPU.
    #[allow(clippy::match_same_arms)]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            VertexAttributeValues::Float32(values) => cast_slice(values),
            VertexAttributeValues::Sint32(values) => cast_slice(values),
            VertexAttributeValues::Uint32(values) => cast_slice(values),
            VertexAttributeValues::Float32x2(values) => cast_slice(values),
            VertexAttributeValues::Sint32x2(values) => cast_slice(values),
            VertexAttributeValues::Uint32x2(values) => cast_slice(values),
            VertexAttributeValues::Float32x3(values) => cast_slice(values),
            VertexAttributeValues::Sint32x3(values) => cast_slice(values),
            VertexAttributeValues::Uint32x3(values) => cast_slice(values),
            VertexAttributeValues::Float32x4(values) => cast_slice(values),
            VertexAttributeValues::Sint32x4(values) => cast_slice(values),
            VertexAttributeValues::Uint32x4(values) => cast_slice(values),
            VertexAttributeValues::Sint16x2(values) => cast_slice(values),
            VertexAttributeValues::Snorm16x2(values) => cast_slice(values),
            VertexAttributeValues::Uint16x2(values) => cast_slice(values),
            VertexAttributeValues::Unorm16x2(values) => cast_slice(values),
            VertexAttributeValues::Sint16x4(values) => cast_slice(values),
            VertexAttributeValues::Snorm16x4(values) => cast_slice(values),
            VertexAttributeValues::Uint16x4(values) => cast_slice(values),
            VertexAttributeValues::Unorm16x4(values) => cast_slice(values),
            VertexAttributeValues::Sint8x2(values) => cast_slice(values),
            VertexAttributeValues::Snorm8x2(values) => cast_slice(values),
            VertexAttributeValues::Uint8x2(values) => cast_slice(values),
            VertexAttributeValues::Unorm8x2(values) => cast_slice(values),
            VertexAttributeValues::Sint8x4(values) => cast_slice(values),
            VertexAttributeValues::Snorm8x4(values) => cast_slice(values),
            VertexAttributeValues::Uint8x4(values) => cast_slice(values),
            VertexAttributeValues::Unorm8x4(values) => cast_slice(values),
        }
    }
}

impl From<&VertexAttributeValues> for wgpu::VertexFormat {
    fn from(values: &VertexAttributeValues) -> Self {
        match values {
            VertexAttributeValues::Float32(_) => wgpu::VertexFormat::Float32,
            VertexAttributeValues::Sint32(_) => wgpu::VertexFormat::Sint32,
            VertexAttributeValues::Uint32(_) => wgpu::VertexFormat::Uint32,
            VertexAttributeValues::Float32x2(_) => wgpu::VertexFormat::Float32x2,
            VertexAttributeValues::Sint32x2(_) => wgpu::VertexFormat::Sint32x2,
            VertexAttributeValues::Uint32x2(_) => wgpu::VertexFormat::Uint32x2,
            VertexAttributeValues::Float32x3(_) => wgpu::VertexFormat::Float32x3,
            VertexAttributeValues::Sint32x3(_) => wgpu::VertexFormat::Sint32x3,
            VertexAttributeValues::Uint32x3(_) => wgpu::VertexFormat::Uint32x3,
            VertexAttributeValues::Float32x4(_) => wgpu::VertexFormat::Float32x4,
            VertexAttributeValues::Sint32x4(_) => wgpu::VertexFormat::Sint32x4,
            VertexAttributeValues::Uint32x4(_) => wgpu::VertexFormat::Uint32x4,
            VertexAttributeValues::Sint16x2(_) => wgpu::VertexFormat::Sint16x2,
            VertexAttributeValues::Snorm16x2(_) => wgpu::VertexFormat::Snorm16x2,
            VertexAttributeValues::Uint16x2(_) => wgpu::VertexFormat::Uint16x2,
            VertexAttributeValues::Unorm16x2(_) => wgpu::VertexFormat::Unorm16x2,
            VertexAttributeValues::Sint16x4(_) => wgpu::VertexFormat::Sint16x4,
            VertexAttributeValues::Snorm16x4(_) => wgpu::VertexFormat::Snorm16x4,
            VertexAttributeValues::Uint16x4(_) => wgpu::VertexFormat::Uint16x4,
            VertexAttributeValues::Unorm16x4(_) => wgpu::VertexFormat::Unorm16x4,
            VertexAttributeValues::Sint8x2(_) => wgpu::VertexFormat::Sint8x2,
            VertexAttributeValues::Snorm8x2(_) => wgpu::VertexFormat::Snorm8x2,
            VertexAttributeValues::Uint8x2(_) => wgpu::VertexFormat::Uint8x2,
            VertexAttributeValues::Unorm8x2(_) => wgpu::VertexFormat::Unorm8x2,
            VertexAttributeValues::Sint8x4(_) => wgpu::VertexFormat::Sint8x4,
            VertexAttributeValues::Snorm8x4(_) => wgpu::VertexFormat::Snorm8x4,
            VertexAttributeValues::Uint8x4(_) => wgpu::VertexFormat::Uint8x4,
            VertexAttributeValues::Unorm8x4(_) => wgpu::VertexFormat::Unorm8x4,
        }
    }
}
