#[derive(Debug)]
pub struct DepthStencilStateBuilder {
    format: wgpu::TextureFormat,
    depth_write_enabled: bool,
    depth_compare: wgpu::CompareFunction,
    stencil: wgpu::StencilState,
    bias: wgpu::DepthBiasState,
}

impl Default for DepthStencilStateBuilder {
    fn default() -> Self {
        Self {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }
    }
}

impl DepthStencilStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn depth_write_enabled(mut self, enabled: bool) -> Self {
        self.depth_write_enabled = enabled;
        self
    }

    pub fn depth_compare(mut self, compare: wgpu::CompareFunction) -> Self {
        self.depth_compare = compare;
        self
    }

    pub fn stencil(mut self, stencil: wgpu::StencilState) -> Self {
        self.stencil = stencil;
        self
    }

    pub fn bias(mut self, bias: wgpu::DepthBiasState) -> Self {
        self.bias = bias;
        self
    }

    pub fn build(self) -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: self.format,
            depth_write_enabled: self.depth_write_enabled,
            depth_compare: self.depth_compare,
            stencil: self.stencil,
            bias: self.bias,
        }
    }

    pub fn build_for_render_bundle(self) -> Option<wgpu::RenderBundleDepthStencil> {
        Some(wgpu::RenderBundleDepthStencil {
            format: self.format,
            depth_read_only: !self.depth_write_enabled,
            stencil_read_only: true,
        })
    }
}
