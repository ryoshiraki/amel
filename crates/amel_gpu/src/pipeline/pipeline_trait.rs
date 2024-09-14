use super::{prelude::*, state::depth_stencil_state};
use std::ops::Deref;

pub trait PipelineTrait<'a>: Deref<Target = wgpu::RenderPipeline> + Sized {
    fn build(
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        blend_state: wgpu::BlendState,
        primitive_topology: wgpu::PrimitiveTopology,
        sample_count: u32,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = Self::pipeline_layout(device);
        let color_target_states = Self::color_target_state(color_format, blend_state);
        let depth_stencil_state = Self::depth_stencil_state(depth_format);

        let shader = Self::shader(device);
        let vertex_buffer_layouts = Self::vertex_buffer_layouts();
        let vertex_state = Self::vertex_state(shader, "vs_main", &vertex_buffer_layouts);
        let fragment_state = Self::fragment_state(shader, "fs_main", &color_target_states);

        RenderPipelineBuilder::from_layout(pipeline_layout, vertex_state)
            .fragment_state(fragment_state)
            .depth_stencil(depth_stencil_state)
            .primitive_topology(primitive_topology)
            .sample_count(sample_count)
            .build(device)
    }

    fn shader(device: &wgpu::Device) -> &'a wgpu::ShaderModule;
    fn bind_group_layouts(device: &wgpu::Device) -> Vec<&wgpu::BindGroupLayout>;
    fn vertex_buffer_layouts() -> Vec<wgpu::VertexBufferLayout<'static>>;

    fn pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        let bind_group_layouts = Self::bind_group_layouts(device);
        PipelineLayoutBuilder::new()
            .add_bindings(&bind_group_layouts)
            .build(device)
    }

    fn vertex_state(
        shader: &'a wgpu::ShaderModule,
        entry_point: &'static str,
        vertex_buffer_layouts: &'a Vec<wgpu::VertexBufferLayout<'static>>,
    ) -> wgpu::VertexState<'a> {
        VertexStateBuilder::new()
            .shader(shader)
            .entry_point(entry_point)
            .buffers(vertex_buffer_layouts)
            .build()
    }

    fn fragment_state(
        shader: &'a wgpu::ShaderModule,
        entry_point: &'static str,
        color_taget_states: &'a Vec<Option<wgpu::ColorTargetState>>,
    ) -> wgpu::FragmentState<'a> {
        FragmentStateBuilder::new()
            .shader(shader)
            .entry_point(entry_point)
            .targets(color_taget_states)
            .build()
    }

    fn color_target_state(
        color_format: wgpu::TextureFormat,
        blend_state: wgpu::BlendState,
    ) -> Vec<Option<wgpu::ColorTargetState>> {
        vec![Some(
            ColorTargetStateBuilder::new()
                .format(color_format)
                .blend(blend_state)
                .build(),
        )]
    }

    fn depth_stencil_state(
        depth_format: Option<wgpu::TextureFormat>,
    ) -> Option<wgpu::DepthStencilState> {
        depth_format.map(|format| {
            DepthStencilStateBuilder::new()
                .format(format)
                .depth_write_enabled(true)
                .depth_compare(wgpu::CompareFunction::Less)
                .build()
        })
    }

    fn to_wgpu(&self) -> &wgpu::RenderPipeline;
}
