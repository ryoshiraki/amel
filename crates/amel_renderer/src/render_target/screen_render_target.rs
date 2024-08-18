use amel_gpu::prelude::*;
use amel_math::prelude::*;

use super::RenderTarget;

pub struct ScreenRenderTarget<'a> {
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture: Option<(wgpu::SurfaceTexture, wgpu::TextureView)>,
    depth_texture: Option<Texture>,
}

impl<'a> ScreenRenderTarget<'a> {
    pub fn new(
        window: &Window,
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
    ) -> Result<Self, wgpu::CreateSurfaceError> {
        let config = window.config();

        let window = window.window_arc().clone();
        let size = window.inner_size();

        let surface = instance.create_surface(window)?;
        let surface_config = surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();
        surface.configure(device, &surface_config);

        // let mut surface: Option<wgpu::Surface<'static>> = None;
        // if cfg!(target_arch = "wasm32") {
        //     surface = Some(instance.create_surface(window).unwrap());
        // }

        let depth_texture = config.depth_format.map(|format| {
            TextureBuilder::new()
                .label("depth_texture")
                .size(config.size.width, config.size.height)
                .format(format)
                .sample_count(1)
                .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
                .build(device)
        });

        Ok(Self {
            // window: window.clone(),
            surface,
            surface_config,
            surface_texture: None,
            depth_texture,
        })
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(device, &self.surface_config);

        if let Some(ref mut depth_texture) = self.depth_texture {
            *depth_texture = TextureBuilder::new()
                .label("depth_texture")
                .size(width, height)
                .format(depth_texture.format())
                .sample_count(1)
                .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
                .build(device);
        }
    }

    pub fn reset(&mut self) {
        self.surface_texture = match self.surface.get_current_texture() {
            Ok(frame) => {
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                Some((frame, view))
            }
            Err(e) => {
                log::error!("Failed to get current surface texture: {:?}", e);
                None
            }
        };
    }

    pub fn present(&mut self) {
        if let Some((frame, _)) = self.surface_texture.take() {
            frame.present();
        }
    }
}

impl<'a> RenderTarget for ScreenRenderTarget<'a> {
    fn color_attachments(&self, op: PassOp<Vec4>) -> Vec<Option<wgpu::RenderPassColorAttachment>> {
        if let Some((_, surface_view)) = &self.surface_texture {
            vec![Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: op.to_wgpu(),
                    store: wgpu::StoreOp::Store,
                },
            })]
        } else {
            vec![]
        }
    }

    fn depth_attachment(&self, op: PassOp<f32>) -> Option<wgpu::RenderPassDepthStencilAttachment> {
        self.depth_texture
            .as_ref()
            .map(|texture| wgpu::RenderPassDepthStencilAttachment {
                view: &texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: op.to_wgpu(),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            })
    }

    // fn render_pass_descriptor(&self) -> wgpu::RenderPassDescriptor {
    //     let color_attachments = self.color_attachments();
    //     let depth_stencil_attachment = self.depth_attachment();

    //     wgpu::RenderPassDescriptor {
    //         label: Some("Offscreen Render Pass"),
    //         color_attachments: &color_attachments,
    //         depth_stencil_attachment,
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     }
    // }

    // fn clear_color_texture(&self, device: &wgpu::Device, queue: &wgpu::Queue, color: wgpu::Color) {}
    // fn clear_depth_texture(&self, device: &wgpu::Device, queue: &wgpu::Queue, depth: f32) {}
}
