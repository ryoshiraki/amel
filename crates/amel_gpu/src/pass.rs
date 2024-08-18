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