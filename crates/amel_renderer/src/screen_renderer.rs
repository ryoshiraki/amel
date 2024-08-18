use amel_gpu::prelude::*;

pub struct ScreenRenderer<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture: Option<(wgpu::SurfaceTexture, wgpu::TextureView)>,
}