use super::attribute::*;
use super::indices::Indices;

use amel_gpu::prelude::*;
use std::collections::HashMap;

pub trait MeshVertex {
    fn desc<'a>(self) -> wgpu::VertexBufferLayout<'a>;
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub primitive_topology: wgpu::PrimitiveTopology,
    pub attributes: HashMap<u32, VertexAttributeValues>,
    pub indices: Option<Indices>,
}

impl Mesh {
    pub const ATTRIBUTE_POSITION: u32 = 0;
    pub const ATTRIBUTE_NORMAL: u32 = 1;
    pub const ATTRIBUTE_UV_0: u32 = 2;
    pub const ATTRIBUTE_UV_1: u32 = 3;
    pub const ATTRIBUTE_TANGENT: u32 = 4;
    pub const ATTRIBUTE_COLOR: u32 = 5;

    pub fn new(primitive_topology: wgpu::PrimitiveTopology) -> Self {
        Self {
            primitive_topology,
            attributes: HashMap::new(),
            indices: None,
        }
    }

    pub fn primitive_topology(&self) -> wgpu::PrimitiveTopology {
        self.primitive_topology
    }

    #[inline]
    pub fn insert_attribute(
        &mut self,
        location: u32,
        attributes: impl Into<VertexAttributeValues>,
    ) {
        let attributes = attributes.into();
        //let values_format = wgpu::VertexFormat::from(&attributes);
        self.attributes.insert(location, attributes);
    }

    #[must_use]
    #[inline]
    pub fn with_inserted_attribute(
        mut self,
        location: u32,
        attributes: impl Into<VertexAttributeValues>,
    ) -> Self {
        self.insert_attribute(location, attributes);
        self
    }

    #[inline]
    pub fn indices(&self) -> Option<&Indices> {
        self.indices.as_ref()
    }

    #[inline]
    pub fn insert_indices<I: Into<Indices>>(&mut self, indices: I) {
        self.indices = Some(indices.into());
    }

    #[must_use]
    #[inline]
    pub fn with_inserted_indices<I: Into<Indices>>(mut self, indices: I) -> Self {
        self.insert_indices(indices.into());
        self
    }

    #[inline]
    pub fn attribute(&self, location: u32) -> Option<&VertexAttributeValues> {
        self.attributes.get(&location)
    }

    #[inline]
    pub fn attribute_mut(&mut self, location: u32) -> Option<&mut VertexAttributeValues> {
        self.attributes.get_mut(&location)
    }
}

// struct VertexAttribute {
//     location: u32,
//     format: wgpu::VertexFormat,
// }

// impl VertexAttribute {
//     pub const fn new(location: u32, format: wgpu::VertexFormat) -> Self {
//         Self { location, format }
//     }
// }

// pub struct Mesh {
//     attributes: BTreeMap<u32, VertexAttribute>,
//     indices: Option<Indices>,
//     primitive_topology: wgpu::PrimitiveTopology,
// }

// impl Mesh {
//     pub const ATTRIBUTE_POSITION: VertexAttribute =
//         VertexAttribute::new(0, wgpu::VertexFormat::Float32x3);

//     pub fn new(primitive_topology: gpu::wgpu::PrimitiveTopology) -> Self {
//         Self {
//             primitive_topology,
//             attributes: BTreeMap::new(),
//             indices: None,
//         }
//     }

//     #[inline]
//     pub fn insert_attribute(&mut self, attribute: VertexAttribute) {
//         self.attributes.insert(attribute.location, attribute);
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
// }
