use std::ops::Deref;

pub trait PipelineTrait: Deref<Target = wgpu::RenderPipeline> + Sized {
    // fn build(
    //     device: &'a wgpu::Device,
    //     queue: &'a wgpu::Queue,
    //     color_target_state: &[Option<wgpu::ColorTargetState>],
    //     depth_stencil_state: Option<wgpu::DepthStencilState>,
    //     primitive_topology: wgpu::PrimitiveTopology,
    //     sample_count: u32,
    // ) -> Self;
    fn build(
        pipeline: &wgpu::RenderPipeline,
    ) -> Self;
}


// impl<'a> Deref for MyPipeline {
//     type Target = wgpu::RenderPipeline;
//     fn deref(&self) -> &Self::Target {
//         &self.pipeline
//     }
// }
