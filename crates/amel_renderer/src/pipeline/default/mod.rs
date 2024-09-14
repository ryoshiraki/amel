use std::{ops::Deref, vec};

use super::Uniforms;
use amel_gpu::prelude::*;

pub struct DefaultPipeline {
    pub pipeline: wgpu::RenderPipeline,
    uniform_buffer: DynamicUniformBuffer,
}

impl DefaultPipeline {
    pub fn update_uniforms(&mut self, queue: &wgpu::Queue, index: usize, data: &Uniforms) {
        self.uniform_buffer.update(queue, index, data);
    }
}

impl<'a> PipelineTrait<'a> for DefaultPipeline {
    fn shader_path() -> &'static str {
        include_str!("default.wgsl")
    }

    fn vertex_entry_point() -> &'static str {
        "vs_main"
    }

    fn fragment_entry_point() -> &'static str {
        "fs_main"
    }

    fn bind_group_layouts(device: &wgpu::Device) -> Vec<wgpu::BindGroupLayout> {
        let vertex_bindgroup_layout = BindGroupLayoutBuilder::new()
            .add_to_vertex_stage(BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            })
            .build(device);

        let fragment_bindgroup_layout = BindGroupLayoutBuilder::new()
            .add_to_fragment_stage(BindingType::Texture2D {
                multisampled: false,
                filterable: true,
            })
            .build(device);

        vec![vertex_bindgroup_layout, fragment_bindgroup_layout]
    }

    fn vertex_attributes() -> Vec<VertexAttributes> {
        let pos_attributes =
            VertexAttributes::new().add_attribute(0, wgpu::VertexFormat::Float32x3);
        let normal_attributes =
            VertexAttributes::new().add_attribute(1, wgpu::VertexFormat::Float32x3);
        let texcoord_attributes: VertexAttributes =
            VertexAttributes::new().add_attribute(2, wgpu::VertexFormat::Float32x2);

        vec![pos_attributes, normal_attributes, texcoord_attributes]
    }
}
