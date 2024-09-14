pub mod attribute;
pub mod conversion;
pub mod gpu_mesh;
pub mod indices;
pub mod mesh;
pub mod primitive;

pub mod prelude {
    pub use super::attribute::*;
    pub use super::conversion::*;
    pub use super::gpu_mesh::*;
    pub use super::indices::*;
    pub use super::mesh::*;
    pub use super::primitive::*;
    // pub use crate::Mesh;
}

// use gpu::prelude::*;
// use indices::Indices;

// #[derive(Debug, Clone)]
// pub struct Mesh {
//     primitive_topology: gpu::wgpu::PrimitiveTopology,
//     attributes: BTreeMap<u32, VertexAttributeData>,
//     indices: Option<Indices>,
// }

// impl Mesh {
//     pub const ATTRIBUTE_POSITION: VertexAttribute =
//         VertexAttribute::new(0, wgpu::VertexFormat::Float32x3);
//     pub const ATTRIBUTE_NORMAL: VertexAttribute =
//         VertexAttribute::new(1, wgpu::VertexFormat::Float32x3);
//     pub const ATTRIBUTE_UV_0: VertexAttribute =
//         VertexAttribute::new(2, wgpu::VertexFormat::Float32x2);
//     pub const ATTRIBUTE_UV_1: VertexAttribute =
//         VertexAttribute::new(3, wgpu::VertexFormat::Float32x2);
//     pub const ATTRIBUTE_TANGENT: VertexAttribute =
//         VertexAttribute::new(4, wgpu::VertexFormat::Float32x4);
//     pub const ATTRIBUTE_COLOR: VertexAttribute =
//         VertexAttribute::new(5, wgpu::VertexFormat::Float32x4);

//     pub fn new(primitive_topology: gpu::wgpu::PrimitiveTopology) -> Self {
//         Self {
//             primitive_topology,
//             attributes: BTreeMap::new(),
//             indices: None,
//         }
//     }

//     pub fn new_from(primitive: impl Into<Self>) -> Self {
//         primitive.into()
//     }

//     pub fn primitive_topology(&self) -> gpu::wgpu::PrimitiveTopology {
//         self.primitive_topology
//     }

//     #[inline]
//     pub fn insert_attribute(
//         &mut self,
//         attribute: VertexAttribute,
//         values: impl Into<VertexAttributeValues>,
//     ) {
//         let values = values.into();
//         let values_format = wgpu::VertexFormat::from(&values);
//         if values_format != attribute.format {
//             panic!(
//                 "Failed to insert attribute. Given format is {values_format:?} but expected {:?}",
//                 attribute.format
//             );
//         }

//         self.attributes
//             .insert(attribute.id, VertexAttributeData { attribute, values });
//     }

//     #[must_use]
//     #[inline]
//     pub fn with_inserted_attribute(
//         mut self,
//         attribute: VertexAttribute,
//         values: impl Into<VertexAttributeValues>,
//     ) -> Self {
//         self.insert_attribute(attribute, values);
//         self
//     }

//     #[inline]
//     pub fn indices(&self) -> Option<&Indices> {
//         self.indices.as_ref()
//     }

//     #[inline]
//     pub fn insert_indices<I: Into<Indices>>(&mut self, indices: I) {
//         self.indices = Some(indices.into());
//     }

//     #[must_use]
//     #[inline]
//     pub fn with_inserted_indices<I: Into<Indices>>(mut self, indices: I) -> Self {
//         self.insert_indices(indices.into());
//         self
//     }

//     #[inline]
//     pub fn attribute_mut(&mut self, id: impl Into<u32>) -> Option<&mut VertexAttributeValues> {
//         self.attributes
//             .get_mut(&id.into())
//             .map(|data| &mut data.values)
//     }

//     pub fn count_vertices(&self) -> usize {
//         let mut vertex_count: Option<usize> = None;
//         for attribute_data in self.attributes.values() {
//             let attribute_len = attribute_data.values.len();
//             if let Some(previous_vertex_count) = vertex_count {
//                 if previous_vertex_count != attribute_len {
//                     // let name = self
//                     //     .attributes
//                     //     .get(attribute_id)
//                     //     .map(|data| data.attribute.name.to_string())
//                     //     .unwrap_or_else(|| format!("{attribute_id:?}"));

//                     // warn!("{name} has a different vertex count ({attribute_len}) than other attributes ({previous_vertex_count}) in this mesh, \
//                     //     all attributes will be truncated to match the smallest.");
//                     vertex_count = Some(std::cmp::min(previous_vertex_count, attribute_len));
//                 }
//             } else {
//                 vertex_count = Some(attribute_len);
//             }
//         }

//         vertex_count.unwrap_or(0)
//     }

//     pub fn get_vertex_buffer_data(&self) -> Vec<u8> {
//         let mut vertex_size = 0;
//         for attribute_data in self.attributes.values() {
//             let vertex_format = attribute_data.attribute.format;
//             vertex_size += vertex_format.size() as usize;
//         }

//         let vertex_count = self.count_vertices();

//         let mut attributes_interleaved_buffer = vec![0; vertex_count * vertex_size];
//         // bundle into interleaved buffers
//         let mut attribute_offset = 0;
//         for attribute_data in self.attributes.values() {
//             let attribute_size = attribute_data.attribute.format.size() as usize;
//             let attributes_bytes = attribute_data.values.get_bytes();
//             for (vertex_index, attribute_bytes) in attributes_bytes
//                 .chunks_exact(attribute_size)
//                 .take(vertex_count)
//                 .enumerate()
//             {
//                 let offset = vertex_index * vertex_size + attribute_offset;
//                 attributes_interleaved_buffer[offset..offset + attribute_size]
//                     .copy_from_slice(attribute_bytes);
//             }

//             attribute_offset += attribute_size;
//         }

//         attributes_interleaved_buffer
//     }
// }
