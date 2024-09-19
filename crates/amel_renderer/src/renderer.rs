use super::render_context::RenderContext;
use amel_gpu::prelude::*;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new<'a, T: PipelineTrait<'a>>(
        device: &'a wgpu::Device,
        color_formats: Vec<Option<wgpu::TextureFormat>>,
        depth_format: Option<wgpu::TextureFormat>,
        blend_state: wgpu::BlendState,
        primitive_topology: wgpu::PrimitiveTopology,
        sample_count: u32,
    ) -> Self {
        let color_target_states = color_formats
            .into_iter()
            .fold(ColorTargetStatesBuilder::new(), |builder, format| {
                builder.add_target(
                    format.map(|fmt| ColorTargetState::new().format(fmt).blend(blend_state)),
                )
            })
            .build();

        let depth_stencil_state = depth_format.map(|format| {
            DepthStencilStateBuilder::new()
                .format(format)
                .depth_write_enabled(true)
                .depth_compare(wgpu::CompareFunction::Less)
                .build()
        });

        let pipeline = T::build(
            device,
            color_target_states,
            depth_stencil_state,
            primitive_topology,
            sample_count,
        );

        Renderer { pipeline }
    }

    pub fn draw<F>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_view: Vec<Option<wgpu::TextureView>>,
        depth_view: Option<wgpu::TextureView>,
        f: F,
    ) where
        F: FnOnce(&mut RenderContext),
    {
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

        let color_attachments = color_view.iter().fold(Vec::new(), |mut acc, view| {
            acc.push(Some(wgpu::RenderPassColorAttachment {
                view: view.as_ref().unwrap(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            }));
            acc
        });

        let depth_stencil_attachment =
            depth_view
                .as_ref()
                .map(|view| wgpu::RenderPassDepthStencilAttachment {
                    view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            let mut context = RenderContext::new(device, queue, &self.pipeline, &mut render_pass);
            f(&mut context);
        }

        queue.submit(Some(command_encoder.finish()));
    }
}

// pub struct Renderer<'a> {
//     pipeline: wgpu::RenderPipeline,
//     render_bundle_encoder: wgpu::RenderBundleEncoder<'a>,
//     render_bundle_depth_stencil: Option<wgpu::RenderBundleDepthStencil>,
//     texture_view: Vec<Arc<wgpu::TextureView>>,
//     depth_view: Option<Arc<wgpu::TextureView>>,
// }

// impl<'a> Renderer<'a> {
//     pub fn new<T: PipelineTrait<'a>>(
//         device: &'a wgpu::Device,
//         color_texture: &'a [&'a Texture],
//         depth_texture: Option<&'a Texture>,
//         blend_state: wgpu::BlendState,
//         primitive_topology: wgpu::PrimitiveTopology,
//         sample_count: u32,
//     ) -> Self {
//         let render_bundle_depth_stencil =
//             depth_texture.map(|depth_texture| wgpu::RenderBundleDepthStencil {
//                 format: depth_texture.format(),
//                 depth_read_only: false,
//                 stencil_read_only: false,
//             });

//         let color_target_states = ColorTargetStatesBuilder::new()
//             .add_target(Some(
//                 ColorTargetState::new()
//                     .format(color_texture[0].format())
//                     .blend(blend_state),
//             ))
//             .build();

//         let depth_stencil_state = depth_texture.map(|texture| {
//             DepthStencilStateBuilder::new()
//                 .format(texture.format())
//                 .depth_write_enabled(true)
//                 .depth_compare(wgpu::CompareFunction::Less)
//                 .build()
//         });

//         let pipeline = T::build(
//             device,
//             color_target_states,
//             depth_stencil_state,
//             primitive_topology,
//             sample_count,
//         );

//         Renderer {
//             pipeline,
//             render_bundle_encoder: device.create_render_bundle_encoder(
//                 &wgpu::RenderBundleEncoderDescriptor {
//                     label: Some("Render Bundle Encoder"),
//                     color_formats: &color_texture
//                         .iter()
//                         .map(|&texture| Some(texture.format()))
//                         .collect::<Vec<_>>(),
//                     depth_stencil: render_bundle_depth_stencil,
//                     sample_count: 1,
//                     multiview: None,
//                 },
//             ),
//             render_bundle_depth_stencil,
//             texture_view: color_texture
//                 .iter()
//                 .map(|&texture| texture.view.clone())
//                 .collect(),
//             depth_view: depth_texture.map(|texture| texture.view.clone()),
//         }
//     }

//     pub fn draw<F>(
//         &mut self,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         color_texture: &[&wgpu::Texture],
//         f: F,
//     ) where
//         F: FnOnce(&mut RenderContext),
//     {
//         let mut render_bundle_encoder =
//             device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
//                 label: Some("Render Bundle Encoder"),
//                 color_formats: &color_texture
//                     .iter()
//                     .map(|&texture| Some(texture.format()))
//                     .collect::<Vec<_>>(),
//                 depth_stencil: self.render_bundle_depth_stencil,
//                 sample_count: 1,
//                 multiview: None,
//             });

//         render_bundle_encoder.set_pipeline(&self.pipeline);
//         let mut context = RenderContext::new(render_bundle_encoder);
//         f(&mut context);

//         let render_bundles = context.finish();

//         let color_attachments = self
//             .texture_view
//             .iter()
//             .map(|view| {
//                 Some(wgpu::RenderPassColorAttachment {
//                     view: view.as_ref(),
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })
//             })
//             .collect::<Vec<_>>();

//         let depth_stencil_attachment =
//             self.depth_view
//                 .as_ref()
//                 .map(|view| wgpu::RenderPassDepthStencilAttachment {
//                     view: view.as_ref(),
//                     depth_ops: Some(wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(1.0),
//                         store: wgpu::StoreOp::Store,
//                     }),
//                     stencil_ops: None,
//                 });

//         let command_buffer = encode(device, |encoder| {
//             let mut render_pass =
//                 wgpu::RenderPass::begin(encoder, &color_attachments, depth_stencil_attachment);

//             for render_bundle in &render_bundles {
//                 render_pass.execute_bundles(std::iter::once(render_bundle));
//             }
//         });

//         queue.submit(std::iter::once(command_buffer));
//     }
// }

// fn encode<F>(device: &wgpu::Device, f: F) -> wgpu::CommandBuffer
// where
//     F: FnOnce(&mut wgpu::CommandEncoder),
// {
//     let mut encoder =
//         device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//     f(&mut encoder);
//     encoder.finish()
// }
