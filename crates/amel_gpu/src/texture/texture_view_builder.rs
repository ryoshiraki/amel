#[derive(Debug, Default)]
pub struct TextureViewBuilder {
    desc: wgpu::TextureViewDescriptor<'static>,
}

impl TextureViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.desc.format = Some(format);
        self
    }

    #[inline]
    pub fn dimension(mut self, dimension: wgpu::TextureViewDimension) -> Self {
        self.desc.dimension = Some(dimension);
        self
    }

    #[inline]
    pub fn aspect(mut self, aspect: wgpu::TextureAspect) -> Self {
        self.desc.aspect = aspect;
        self
    }

    #[inline]
    pub fn mip_level_count(mut self, mip_level_count: Option<u32>) -> Self {
        self.desc.mip_level_count = mip_level_count;
        self
    }

    #[inline]
    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.desc.base_array_layer = base_array_layer;
        self
    }

    #[inline]
    pub fn array_layer_count(mut self, array_layer_count: Option<u32>) -> Self {
        self.desc.array_layer_count = array_layer_count;
        self
    }

    #[inline]
    pub fn build(self, texture: &wgpu::Texture) -> wgpu::TextureView {
        texture.create_view(&self.desc)
    }
}
