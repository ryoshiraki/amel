use super::prelude::*;

pub trait PipelineTrait<'a> {
    fn build(
        device: &'a wgpu::Device,
        color_target_states: Vec<Option<wgpu::ColorTargetState>>,
        depth_stencil_state: Option<wgpu::DepthStencilState>,
        primitive_topology: wgpu::PrimitiveTopology,
        sample_count: u32,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = Self::pipeline_layout(device);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Self::shader_path().into()),
        });

        let vertex_attributes = Self::vertex_attributes();
        let vertex_buffer_layouts = vertex_attributes
            .iter()
            .map(|attr| attr.vertex_buffer_layout())
            .collect();

        let vertex_state = VertexStateBuilder::new()
            .shader(&shader)
            .entry_point(Self::vertex_entry_point())
            .buffers(&vertex_buffer_layouts)
            .build();

        let fragment_state = FragmentStateBuilder::new()
            .shader(&shader)
            .entry_point(Self::fragment_entry_point())
            .targets(&color_target_states)
            .build();

        RenderPipelineBuilder::from_layout(pipeline_layout, vertex_state)
            .fragment_state(fragment_state)
            .depth_stencil(depth_stencil_state)
            .primitive_topology(primitive_topology)
            .sample_count(sample_count)
            .build(device)
    }

    fn shader_path() -> &'static str;

    fn vertex_entry_point() -> &'static str {
        "main"
    }
    fn fragment_entry_point() -> &'static str {
        "main"
    }

    fn bind_group_layouts(device: &wgpu::Device) -> Vec<wgpu::BindGroupLayout>;

    fn vertex_attributes() -> Vec<VertexAttributes>;

    fn pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        let bind_group_layouts = Self::bind_group_layouts(device);
        let layouts_refs: Vec<&wgpu::BindGroupLayout> = bind_group_layouts.iter().collect(); //
        PipelineLayoutBuilder::new()
            .add_bindings(&layouts_refs)
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
        color_taget_states: &'a [Option<wgpu::ColorTargetState>],
    ) -> wgpu::FragmentState<'a> {
        FragmentStateBuilder::new()
            .shader(shader)
            .entry_point(entry_point)
            .targets(color_taget_states)
            .build()
    }
}
