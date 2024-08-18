use std::{cell::RefCell, rc::Rc, sync::Arc};

use amel_gpu::prelude::*;


struct RenderContext<'a> {
    encoder: &'a wgpu::RenderBundleEncoder<'a>,
}

impl<'a> RenderContext<'a> {
    fn new(encoder: &'a wgpu::RenderBundleEncoder<'a>) -> Self {
        // encoder.set_pipeline(&self.render_resources.pipeline);
        RenderContext {
            encoder,
        }
    }
}

pub struct Renderer<'a> {
    desc: wgpu::RenderBundleEncoderDescriptor<'a>,
    bundles: Vec<wgpu::RenderBundle>,

    // encoder: Rc<RefCell<wgpu::RenderBundleEncoder<'a>>>,
    // pipeline: wgpu::RenderPipeline,
}

impl<'a> Renderer<'a> {
    pub fn new(
        color_formats: &[wgpu::TextureFormat],
        depth_format: Option<wgpu::TextureFormat>,
        sample_count: u32,
    ) -> Self {
        let depth_stencil = depth_format.map(|depth_format| wgpu::RenderBundleDepthStencil {
            format: depth_format,
            depth_read_only: false,
            stencil_read_only: false,
        });

        let color_formats: Vec<Option<wgpu::TextureFormat>> = color_formats.iter().map(|&f| Some(f)).collect();

        let desc = wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder"),
                color_formats: &color_formats,
                depth_stencil,
                sample_count,
                multiview: None,
            };
        

        Self {
            desc,
            bundles: Vec::new(),
        }
    }

    // pub fn set_pipeline(&mut self, pipeline: &'a wgpu::RenderPipeline) {
    //     self.encoder.set_pipeline(pipeline);
    // }

    pub fn render<F>(
        &mut self,
        render_func: F,
    ) 
    where
        F: FnOnce(&mut RenderContext)
    {
        let mut encoder = device.create_render_bundle_encoder(
            &wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder"),
                color_formats: &color_formats.iter().map(|&f| Some(f)).collect::<Vec<_>>(),
                depth_stencil,
                sample_count,
                multiview: None,
            },
        );
        encoder.set_pipeline(pipeline);

        // {
        //     let mut context = RenderContext::new(&self.encoder);
        //     render_func(&mut context);
        // }

        let bundle = self.encoder.clone().finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Bundle"),
        });
        self.bundles.push(bundle);
    }
}
