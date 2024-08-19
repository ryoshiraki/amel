use super::Texture;

#[derive(Debug)]
pub struct TextureBuilder {
    desc: wgpu::TextureDescriptor<'static>,
}

impl Default for TextureBuilder {
    fn default() -> Self {
        Self {
            desc: Self::DEFAULT_DESCRIPTOR,
        }
    }
}

impl TextureBuilder {
    pub const DEFAULT_SIDE: u32 = 128;
    pub const DEFAULT_DEPTH: u32 = 1;
    pub const DEFAULT_SIZE: wgpu::Extent3d = wgpu::Extent3d {
        width: Self::DEFAULT_SIDE,
        height: Self::DEFAULT_SIDE,
        depth_or_array_layers: Self::DEFAULT_DEPTH,
    };
    pub const DEFAULT_ARRAY_LAYER_COUNT: u32 = 1;
    pub const DEFAULT_MIP_LEVEL_COUNT: u32 = 1;
    pub const DEFAULT_SAMPLE_COUNT: u32 = 1;
    pub const DEFAULT_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D2;
    pub const DEFAULT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
    pub const DEFAULT_USAGE: wgpu::TextureUsages = wgpu::TextureUsages::all(); // TODO: is this the right choice?
    pub const DEFAULT_DESCRIPTOR: wgpu::TextureDescriptor<'static> = wgpu::TextureDescriptor {
        label: Some("Texture"),
        size: Self::DEFAULT_SIZE,
        mip_level_count: Self::DEFAULT_MIP_LEVEL_COUNT,
        sample_count: Self::DEFAULT_SAMPLE_COUNT,
        dimension: Self::DEFAULT_DIMENSION,
        format: Self::DEFAULT_FORMAT,
        usage: Self::DEFAULT_USAGE,
        view_formats: &[],
    };

    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn label(mut self, label: &'static str) -> Self {
        self.desc.label = Some(label);
        self
    }

    #[inline]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.desc.size.width = width;
        self.desc.size.height = height;
        self.infer_dimension_from_size();
        self
    }

    #[inline]
    pub fn depth(mut self, depth: u32) -> Self {
        self.desc.size.depth_or_array_layers = depth;
        self.infer_dimension_from_size();
        self
    }

    #[inline]
    pub fn extent(mut self, extent: wgpu::Extent3d) -> Self {
        self.desc.size = extent;
        self.infer_dimension_from_size();
        self
    }

    #[inline]
    pub fn dimension(mut self, dimension: wgpu::TextureDimension) -> Self {
        self.desc.dimension = dimension;
        self
    }

    #[inline]
    pub fn mip_level_count(mut self, count: u32) -> Self {
        self.desc.mip_level_count = count;
        self
    }

    /// Specify the number of samples per pixel in the case that the texture is multisampled.
    #[inline]
    pub fn sample_count(mut self, count: u32) -> Self {
        self.desc.sample_count = count;
        self
    }

    /// Specify the texture format.
    #[inline]
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.desc.format = format;
        self
    }

    #[inline]
    pub fn usage(mut self, usage: wgpu::TextureUsages) -> Self {
        self.desc.usage = usage;
        self
    }

    #[inline]
    fn infer_dimension_from_size(&mut self) {
        if self.desc.size.depth_or_array_layers > 1 {
            self.desc.dimension = wgpu::TextureDimension::D3;
        } else if self.desc.size.height > 1 {
            self.desc.dimension = wgpu::TextureDimension::D2;
        } else {
            self.desc.dimension = wgpu::TextureDimension::D1;
        }
    }

    pub fn build(&self, device: &wgpu::Device) -> Texture {
        Texture::new(device, &self.desc)
    }

    // pub fn build_as_render_target(
    //     &self,
    //     device: &wgpu::Device,
    //     depth_format: Option<wgpu::TextureFormat>,
    // ) -> OffscreenRenderTarget {
    //     let mut color_desc = self.desc.clone();
    //     color_desc.usage |=
    //         wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
    //     let color_texture = Texture::new(device, &color_desc);

    //     let depth_texture = depth_format.map(|format| {
    //         let mut depth_desc = self.desc.clone();
    //         depth_desc.format = format;
    //         depth_desc.usage |=
    //             wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
    //         depth_desc.mip_level_count = 1;
    //         Texture::new(device, &depth_desc)
    //     });

    //     OffscreenRenderTarget::new(color_texture, depth_texture)
    // }
}
