use std::ops::Deref;

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

impl<'a> AbstractPipeline<'a> for DefaultPipeline {
    fn build(
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        color_targets: &[Option<wgpu::ColorTargetState>],
        depth_stencil_state: Option<wgpu::DepthStencilState>,
        primitive_topology: wgpu::PrimitiveTopology,
        sample_count: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("RgbaPipeline"),
            source: wgpu::ShaderSource::Wgsl(include_str!("default.wgsl").into()),
        });

        let bind_group_layout = BindGroupLayoutBuilder::new()
            .add_to_vertex_stage(BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            })
            .build(device);

        let pipeline_layout = PipelineLayoutBuilder::new()
            .add_binding(&bind_group_layout)
            // .add_push_constant_range(wgpu::PushConstantRange {
            //     stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
            //     range: 0..16,
            // })
            .build(device);

        // let vertex_attributes = VertexAttributes::new()
        //     .add_attribute(0, wgpu::VertexFormat::Float32x3)
        //     .add_attribute(1, wgpu::VertexFormat::Float32x3)
        //     .add_attribute(2, wgpu::VertexFormat::Float32x2);

        let pos_attributes: VertexAttributes =
            VertexAttributes::new().add_attribute(0, wgpu::VertexFormat::Float32x3);
        let normal_attributes: VertexAttributes =
            VertexAttributes::new().add_attribute(1, wgpu::VertexFormat::Float32x3);
        let texcoord_attributes: VertexAttributes =
            VertexAttributes::new().add_attribute(2, wgpu::VertexFormat::Float32x2);

        let vertex_buffer_layouts = VertexBufferLayoutsBuilder::new()
            .add_attributes(&pos_attributes)
            .add_attributes(&normal_attributes)
            .add_attributes(&texcoord_attributes)
            .build();

        let vertex_state = VertexStateBuilder::new()
            .shader(&shader)
            .entry_point("vs_main")
            .buffers(&vertex_buffer_layouts)
            .build();

        let fragment_state = FragmentStateBuilder::new()
            .shader(&shader)
            .entry_point("fs_main")
            .targets(color_targets)
            .build();

        RenderPipelineBuilder::from_layout(pipeline_layout, vertex_state)
            .fragment_state(fragment_state)
            .depth_stencil(depth_stencil_state)
            .primitive_topology(primitive_topology)
            .sample_count(sample_count)
            .build(device, queue)
    }

    fn to_wgpu(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
    // fn setup(device: &wgpu::Device, queue: &wgpu::Queue, pipeline: wgpu::RenderPipeline) -> Self {
    //     let uniform_buffer = DynamicUniformBuffer::new::<Uniforms>(
    //         device,
    //         wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    //         std::mem::size_of::<Uniforms>(),
    //         256,
    //     );

    //     Self {
    //         pipeline,
    //         uniform_buffer,
    //     }
    // }

    // fn pipeline(&self) -> &wgpu::RenderPipeline {
    //     &self.pipeline
    // }

    // fn bind_group(&self, device: &wgpu::Device, index: usize) -> wgpu::BindGroup {
    //     BindGroupBuilder::new()
    //         .add_entry(self.uniform_buffer.binding(index))
    //         .build(device, &self.pipeline.get_bind_group_layout(0))
    // }
}

impl Deref for DefaultPipeline {
    type Target = wgpu::RenderPipeline;
    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}
