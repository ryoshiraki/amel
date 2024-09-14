use amel_math::prelude::*;

pub trait RenderPassExt<'a> {
    fn begin(
        encoder: &'a mut wgpu::CommandEncoder,
        color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) -> Self;
}

impl<'a> RenderPassExt<'a> for wgpu::RenderPass<'a> {
    fn begin(
        encoder: &'a mut wgpu::CommandEncoder,
        color_attachments: &'a [Option<wgpu::RenderPassColorAttachment<'a>>],
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) -> Self {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments,
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }
}

#[derive(Debug)]
pub enum PassOp<T> {
    Clear(T),
    Load(),
}

impl PassOp<Vec4> {
    pub fn to_wgpu(&self) -> wgpu::LoadOp<wgpu::Color> {
        match self {
            PassOp::Clear(vec) => {
                let color = wgpu::Color {
                    r: vec.x as f64,
                    g: vec.y as f64,
                    b: vec.z as f64,
                    a: vec.w as f64,
                };
                wgpu::LoadOp::Clear(color)
            }
            PassOp::Load() => wgpu::LoadOp::Load,
        }
    }
}

impl PassOp<f32> {
    pub fn to_wgpu(&self) -> wgpu::LoadOp<f32> {
        match self {
            PassOp::Clear(value) => wgpu::LoadOp::Clear(*value),
            PassOp::Load() => wgpu::LoadOp::Load,
        }
    }
}
