use super::render_context::RenderContext;
use amel_gpu::{
    pipeline::{self, state::depth_stencil_state},
    prelude::*,
};
use std::sync::Arc;
use wgpu::core::device::queue;

pub struct Renderer<'a, T>
where
    T: AbstractPipeline<'a>,
{
    pipeline: T,
    render_bundle_encoder: wgpu::RenderBundleEncoder<'a>,
    render_bundle_depth_stencil: Option<wgpu::RenderBundleDepthStencil>,
    render_bundles: Vec<wgpu::RenderBundle>,
    texture_view: Vec<Arc<wgpu::TextureView>>,
    depth_view: Option<Arc<wgpu::TextureView>>,
}

impl<'a, T: AbstractPipeline<'a>> Renderer<'a, T> {
    pub fn new(
        device: &'a wgpu::Device,
        pipeline: T,
        color_texture: &[&Texture],
        depth_texture: Option<&Texture>,
        blending: wgpu::BlendState,
    ) -> Self {
        let render_bundle_depth_stencil =
            depth_texture.map(|depth_texture| wgpu::RenderBundleDepthStencil {
                format: depth_texture.format(),
                depth_read_only: false,
                stencil_read_only: false,
            });

        let color_target = ColorTargetStateBuilder::new()
            .format(color_texture[0].format())
            .blend(blending)
            .build();

        let depth_stencil: Option<wgpu::DepthStencilState> = depth_texture.map(|texture| {
            DepthStencilStateBuilder::new()
                .format(texture.format())
                .depth_write_enabled(true)
                .depth_compare(wgpu::CompareFunction::Less)
                .build()
        });

        Renderer {
            pipeline,
            render_bundle_encoder: device.create_render_bundle_encoder(
                &wgpu::RenderBundleEncoderDescriptor {
                    label: Some("Render Bundle Encoder"),
                    color_formats: &color_texture
                        .iter()
                        .map(|&texture| Some(texture.format()))
                        .collect::<Vec<_>>(),
                    depth_stencil: render_bundle_depth_stencil,
                    sample_count: 1,
                    multiview: None,
                },
            ),
            render_bundle_depth_stencil,
            render_bundles: Vec::new(),
            texture_view: color_texture
                .iter()
                .map(|&texture| texture.view.clone())
                .collect(),
            depth_view: depth_texture.map(|texture| texture.view.clone()),
        }
    }

    pub fn draw<F>(
        &mut self,
        device: &wgpu::Device,
        color_texture: &[&wgpu::Texture],
        depth_texture: Option<&wgpu::Texture>,
        f: F,
    ) where
        F: FnOnce(&mut RenderContext),
    {
        let depth_stencil_bundle =
            depth_texture.map(|depth_texture| wgpu::RenderBundleDepthStencil {
                format: depth_texture.format(),
                depth_read_only: false,
                stencil_read_only: false,
            });

        let mut render_bundle_encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder"),
                color_formats: &color_texture
                    .iter()
                    .map(|&texture| Some(texture.format()))
                    .collect::<Vec<_>>(),
                depth_stencil: depth_stencil_bundle,
                sample_count: 1,
                multiview: None,
            });

        render_bundle_encoder.set_pipeline(self.pipeline.to_wgpu());
        let mut context = RenderContext::new(render_bundle_encoder);
        f(&mut context);

        let render_bundles = context.finish();
        // .finish(&wgpu::RenderBundleDescriptor {
        //     label: Some("Render Bundle"),
        // });

        self.render_bundles.extend(render_bundles);
    }

    pub fn finish(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let color_attachments = self
            .texture_view
            .iter()
            .map(|view| {
                Some(wgpu::RenderPassColorAttachment {
                    view: view.as_ref(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect::<Vec<_>>();

        let depth_stencil_attachment =
            self.depth_view
                .as_ref()
                .map(|view| wgpu::RenderPassDepthStencilAttachment {
                    view: view.as_ref(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                });

        let command_buffer = encode(device, |encoder| {
            let mut render_pass =
                wgpu::RenderPass::begin(encoder, &color_attachments, depth_stencil_attachment);

            for render_bundle in &self.render_bundles {
                render_pass.execute_bundles(std::iter::once(render_bundle));
            }
        });

        queue.submit(std::iter::once(command_buffer));
    }
}

fn encode<F>(device: &wgpu::Device, f: F) -> wgpu::CommandBuffer
where
    F: FnOnce(&mut wgpu::CommandEncoder),
{
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    f(&mut encoder);
    encoder.finish()
}
