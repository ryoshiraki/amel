use amel_gpu::prelude::*;
use amel_math::prelude::*;

pub mod screen_render_target;

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


pub trait RenderTarget {
    fn color_formats(&self) -> Vec<Option<wgpu::TextureFormat>>;
    fn depth_stencil_format(&self) -> Option<wgpu::TextureFormat>;
    fn color_attachments(&self, op: PassOp<Vec4>) -> Vec<Option<wgpu::RenderPassColorAttachment>>;
    fn depth_attachment(&self, op: PassOp<f32>) -> Option<wgpu::RenderPassDepthStencilAttachment>;
    // fn render_pass_descriptor(&self) -> wgpu::RenderPassDescriptor;
    // fn clear_color_texture(&self, device: &wgpu::Device, queue: &wgpu::Queue, color: wgpu::Color);
    // fn clear_depth_texture(&self, device: &wgpu::Device, queue: &wgpu::Queue, depth: f32);
}