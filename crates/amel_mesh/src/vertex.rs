// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum VertexFormat {
//     // Float,
//     // Float2,
//     // Float3,
//     // Float4,
//     // UByte4,
//     Uint8x2,
//     Uint8x4,
//     Sint8x2,
//     Sint8x4,
//     Unorm8x2,
//     Unorm8x4,
//     Snorm8x2,
//     Snorm8x4,
//     Uint16x2,
//     Uint16x4,
//     Sint16x2,
//     Sint16x4,
//     Unorm16x2,
//     Unorm16x4,
//     Snorm16x2,
//     Snorm16x4,
//     Float16x2,
//     Float16x4,
//     Float32,
//     Float32x2,
//     Float32x3,
//     Float32x4,
//     Uint32,
//     Uint32x2,
//     Uint32x3,
//     Uint32x4,
//     Sint32,
//     Sint32x2,
//     Sint32x3,
//     Sint32x4,
//     Float64,
//     Float64x2,
//     Float64x3,
//     Float64x4,
// }

// impl VertexFormat {
//     #[allow(clippy::match_same_arms)]
//     pub const fn bytesize(self) -> usize {
//         match self {
//             // VertexFormat::Float => 4,
//             // VertexFormat::Float2 => 8,
//             // VertexFormat::Float3 => 12,
//             // VertexFormat::Float4 => 16,
//             // VertexFormat::UByte4 => 4,
//             VertexFormat::Uint8x2 => 2,
//             VertexFormat::Uint8x4 => 4,
//             VertexFormat::Sint8x2 => 2,
//             VertexFormat::Sint8x4 => 4,
//             VertexFormat::Unorm8x2 => 2,
//             VertexFormat::Unorm8x4 => 4,
//             VertexFormat::Snorm8x2 => 2,
//             VertexFormat::Snorm8x4 => 4,
//             VertexFormat::Uint16x2 => 2 * 2,
//             VertexFormat::Uint16x4 => 2 * 4,
//             VertexFormat::Sint16x2 => 2 * 2,
//             VertexFormat::Sint16x4 => 2 * 4,
//             VertexFormat::Unorm16x2 => 2 * 2,
//             VertexFormat::Unorm16x4 => 2 * 4,
//             VertexFormat::Snorm16x2 => 2 * 2,
//             VertexFormat::Snorm16x4 => 2 * 4,
//             VertexFormat::Float16x2 => 2 * 2,
//             VertexFormat::Float16x4 => 2 * 4,
//             VertexFormat::Float32 => 4,
//             VertexFormat::Float32x2 => 4 * 2,
//             VertexFormat::Float32x3 => 4 * 3,
//             VertexFormat::Float32x4 => 4 * 4,
//             VertexFormat::Uint32 => 4,
//             VertexFormat::Uint32x2 => 4 * 2,
//             VertexFormat::Uint32x3 => 4 * 3,
//             VertexFormat::Uint32x4 => 4 * 4,
//             VertexFormat::Sint32 => 4,
//             VertexFormat::Sint32x2 => 4 * 2,
//             VertexFormat::Sint32x3 => 4 * 3,
//             VertexFormat::Sint32x4 => 4 * 4,
//             VertexFormat::Float64 => 8,
//             VertexFormat::Float64x2 => 8 * 2,
//             VertexFormat::Float64x3 => 8 * 3,
//             VertexFormat::Float64x4 => 8 * 4,
//         }
//     }

//     pub const fn to_wgpu(self) -> wgpu::VertexFormat {
//         match self {
//             // VertexFormat::Float => wgpu::VertexFormat::Float32,
//             // VertexFormat::Float2 => wgpu::VertexFormat::Float32x2,
//             // VertexFormat::Float3 => wgpu::VertexFormat::Float32x3,
//             // VertexFormat::Float4 => wgpu::VertexFormat::Float32x4,
//             // VertexFormat::UByte4 => wgpu::VertexFormat::Unorm8x4,
//             VertexFormat::Uint8x2 => wgpu::VertexFormat::Uint8x2,
//             VertexFormat::Uint8x4 => wgpu::VertexFormat::Uint8x4,
//             VertexFormat::Sint8x2 => wgpu::VertexFormat::Sint8x2,
//             VertexFormat::Sint8x4 => wgpu::VertexFormat::Sint8x4,
//             VertexFormat::Unorm8x2 => wgpu::VertexFormat::Unorm8x2,
//             VertexFormat::Unorm8x4 => wgpu::VertexFormat::Unorm8x4,
//             VertexFormat::Snorm8x2 => wgpu::VertexFormat::Snorm8x2,
//             VertexFormat::Snorm8x4 => wgpu::VertexFormat::Snorm8x4,
//             VertexFormat::Uint16x2 => wgpu::VertexFormat::Uint16x2,
//             VertexFormat::Uint16x4 => wgpu::VertexFormat::Uint16x4,
//             VertexFormat::Sint16x2 => wgpu::VertexFormat::Sint16x2,
//             VertexFormat::Sint16x4 => wgpu::VertexFormat::Sint16x4,
//             VertexFormat::Unorm16x2 => wgpu::VertexFormat::Unorm16x2,
//             VertexFormat::Unorm16x4 => wgpu::VertexFormat::Unorm16x4,
//             VertexFormat::Snorm16x2 => wgpu::VertexFormat::Snorm16x2,
//             VertexFormat::Snorm16x4 => wgpu::VertexFormat::Snorm16x4,
//             VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
//             VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
//             VertexFormat::Float32 => wgpu::VertexFormat::Float32,
//             VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
//             VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
//             VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
//             VertexFormat::Uint32 => wgpu::VertexFormat::Uint32,
//             VertexFormat::Uint32x2 => wgpu::VertexFormat::Uint32x2,
//             VertexFormat::Uint32x3 => wgpu::VertexFormat::Uint32x3,
//             VertexFormat::Uint32x4 => wgpu::VertexFormat::Uint32x4,
//             VertexFormat::Sint32 => wgpu::VertexFormat::Sint32,
//             VertexFormat::Sint32x2 => wgpu::VertexFormat::Sint32x2,
//             VertexFormat::Sint32x3 => wgpu::VertexFormat::Sint32x3,
//             VertexFormat::Sint32x4 => wgpu::VertexFormat::Sint32x4,
//             VertexFormat::Float64 => wgpu::VertexFormat::Float64,
//             VertexFormat::Float64x2 => wgpu::VertexFormat::Float64x2,
//             VertexFormat::Float64x3 => wgpu::VertexFormat::Float64x3,
//             VertexFormat::Float64x4 => wgpu::VertexFormat::Float64x4,
//         }
//     }
// }

// /// Describes a 'VertexBuffer' layout.
// #[derive(Default, Debug)]
// pub struct VertexLayout {
//     attributes: Vec<wgpu::VertexAttribute>,
//     size: usize,
// }

// impl VertexLayout {
//     pub fn from(formats: &[VertexFormat]) -> Self {
//         let mut vl = Self::default();
//         for vf in formats {
//             vl.attributes.push(wgpu::VertexAttribute {
//                 shader_location: vl.attributes.len() as u32,
//                 offset: vl.size as wgpu::BufferAddress,
//                 format: vf.to_wgpu(),
//             });
//             vl.size += vf.bytesize();
//         }
//         vl
//     }

//     pub fn to_wgpu(&self) -> wgpu::VertexBufferLayout {
//         wgpu::VertexBufferLayout {
//             array_stride: self.size as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: self.attributes.as_slice(),
//         }
//     }
// }
