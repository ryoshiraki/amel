use amel_gpu::prelude::*;
use amel_math::prelude::*;

pub trait RenderTarget {
    fn bind(&self, encoder: &mut wgpu::CommandEncoder);
    fn unbind(&self, encoder: &mut wgpu::CommandEncoder);

    // fn color_format(&self) -> Vec<Option<wgpu::TextureFormat>>;
    // fn depth_stencil_format(&self) -> Option<wgpu::TextureFormat>;
    fn color_attachment(&self, op: PassOp<Vec4>) -> Vec<Option<wgpu::RenderPassColorAttachment>>;
    fn depth_stencil_attachment(
        &self,
        op: PassOp<f32>,
    ) -> Option<wgpu::RenderPassDepthStencilAttachment>;
}
