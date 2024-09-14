#[derive(Debug)]
pub struct RenderPipelineBuilder<'a> {
    layout: wgpu::PipelineLayout,
    vertex: wgpu::VertexState<'a>,
    fragment: Option<wgpu::FragmentState<'a>>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    // _marker: std::marker::PhantomData<&'a T>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub const DEFAULT_FRONT_FACE: wgpu::FrontFace = wgpu::FrontFace::Ccw;
    pub const DEFAULT_CULL_MODE: Option<wgpu::Face> = None;
    pub const DEFAULT_POLYGON_MODE: wgpu::PolygonMode = wgpu::PolygonMode::Fill;
    pub const DEFAULT_PRIMITIVE_TOPOLOGY: wgpu::PrimitiveTopology =
        wgpu::PrimitiveTopology::TriangleList;
    pub const DEFAULT_PRIMITIVE: wgpu::PrimitiveState = wgpu::PrimitiveState {
        topology: Self::DEFAULT_PRIMITIVE_TOPOLOGY,
        strip_index_format: None,
        front_face: Self::DEFAULT_FRONT_FACE,
        cull_mode: Self::DEFAULT_CULL_MODE,
        polygon_mode: Self::DEFAULT_POLYGON_MODE,
        unclipped_depth: Self::DEFAULT_UNCLIPPED_DEPTH,
        conservative: false,
    };

    // Depth state defaults.
    pub const DEFAULT_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    pub const DEFAULT_DEPTH_WRITE_ENABLED: bool = true;
    pub const DEFAULT_DEPTH_COMPARE: wgpu::CompareFunction = wgpu::CompareFunction::LessEqual;
    pub const DEFAULT_STENCIL_FRONT: wgpu::StencilFaceState = wgpu::StencilFaceState::IGNORE;
    pub const DEFAULT_STENCIL_BACK: wgpu::StencilFaceState = wgpu::StencilFaceState::IGNORE;
    pub const DEFAULT_STENCIL_READ_MASK: u32 = 0;
    pub const DEFAULT_STENCIL_WRITE_MASK: u32 = 0;
    pub const DEFAULT_STENCIL: wgpu::StencilState = wgpu::StencilState {
        front: Self::DEFAULT_STENCIL_FRONT,
        back: Self::DEFAULT_STENCIL_BACK,
        read_mask: Self::DEFAULT_STENCIL_READ_MASK,
        write_mask: Self::DEFAULT_STENCIL_WRITE_MASK,
    };
    pub const DEFAULT_DEPTH_BIAS_CONSTANT: i32 = 0;
    pub const DEFAULT_DEPTH_BIAS_SLOPE_SCALE: f32 = 0.0;
    pub const DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0.0;
    pub const DEFAULT_DEPTH_BIAS: wgpu::DepthBiasState = wgpu::DepthBiasState {
        constant: Self::DEFAULT_DEPTH_BIAS_CONSTANT,
        slope_scale: Self::DEFAULT_DEPTH_BIAS_SLOPE_SCALE,
        clamp: Self::DEFAULT_DEPTH_BIAS_CLAMP,
    };
    pub const DEFAULT_UNCLIPPED_DEPTH: bool = false;
    pub const DEFAULT_DEPTH_STENCIL: wgpu::DepthStencilState = wgpu::DepthStencilState {
        format: Self::DEFAULT_DEPTH_FORMAT,
        depth_write_enabled: Self::DEFAULT_DEPTH_WRITE_ENABLED,
        depth_compare: Self::DEFAULT_DEPTH_COMPARE,
        stencil: Self::DEFAULT_STENCIL,
        bias: Self::DEFAULT_DEPTH_BIAS,
    };

    // Multisample state.
    pub const DEFAULT_SAMPLE_COUNT: u32 = 1;
    pub const DEFAULT_SAMPLE_MASK: u64 = !0;
    pub const DEFAULT_ALPHA_TO_COVERAGE_ENABLED: bool = false;
    pub const DEFAULT_MULTISAMPLE: wgpu::MultisampleState = wgpu::MultisampleState {
        count: Self::DEFAULT_SAMPLE_COUNT,
        mask: Self::DEFAULT_SAMPLE_MASK,
        alpha_to_coverage_enabled: Self::DEFAULT_ALPHA_TO_COVERAGE_ENABLED,
    };

    pub fn from_layout(layout: wgpu::PipelineLayout, vertex: wgpu::VertexState<'a>) -> Self {
        Self {
            layout,
            vertex,
            fragment: None,
            primitive: Self::DEFAULT_PRIMITIVE,
            depth_stencil: None,
            multisample: Self::DEFAULT_MULTISAMPLE,
            // _marker: std::marker::PhantomData,
        }
    }

    pub fn fragment_state(mut self, fragment: wgpu::FragmentState<'a>) -> Self {
        self.fragment = Some(fragment);
        self
    }

    pub fn primitive(mut self, p: wgpu::PrimitiveState) -> Self {
        self.primitive = p;
        self
    }

    pub fn primitive_topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.primitive.topology = topology;
        self
    }

    #[inline]
    pub fn front_face(mut self, front_face: wgpu::FrontFace) -> Self {
        self.primitive.front_face = front_face;
        self
    }

    #[inline]
    pub fn cull_mode(mut self, cull_mode: Option<wgpu::Face>) -> Self {
        self.primitive.cull_mode = cull_mode;
        self
    }

    #[inline]
    pub fn polygon_mode(mut self, mode: wgpu::PolygonMode) -> Self {
        self.primitive.polygon_mode = mode;
        self
    }

    /// Specify the full depth stencil state.
    #[inline]
    pub fn depth_stencil(mut self, state: Option<wgpu::DepthStencilState>) -> Self {
        self.depth_stencil = state;
        self
    }

    /// Format of the depth/stencil buffer. Must be one of the depth formats. Must match the format
    /// of the depth/stencil attachment.
    // #[inline]
    // pub fn depth_format(mut self, format: wgpu::TextureFormat) -> Self {
    //     let state = self
    //         .depth_stencil
    //         .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
    //     state.format = format;
    //     self
    // }

    #[inline]
    pub fn depth_format(mut self, format: Option<wgpu::TextureFormat>) -> Self {
        match format {
            Some(f) => {
                let state = self
                    .depth_stencil
                    .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
                state.format = f;
            }
            None => {
                self.depth_stencil = None;
            }
        }
        self
    }

    #[inline]
    pub fn depth_write_enabled(mut self, enabled: bool) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.depth_write_enabled = enabled;
        self
    }

    /// Comparison function used to compare depth values in the depth test.
    #[inline]
    pub fn depth_compare(mut self, compare: wgpu::CompareFunction) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.depth_compare = compare;
        self
    }

    /// Specify the full set of stencil parameters.
    #[inline]
    pub fn stencil(mut self, stencil: wgpu::StencilState) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.stencil = stencil;
        self
    }

    /// Front face mode.
    #[inline]
    pub fn stencil_front(mut self, stencil: wgpu::StencilFaceState) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.stencil.front = stencil;
        self
    }

    /// Back face mode.
    #[inline]
    pub fn stencil_back(mut self, stencil: wgpu::StencilFaceState) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.stencil.back = stencil;
        self
    }

    /// Stencil values are AND'd with this mask when reading and writing from the stencil buffer.
    /// Only low 8 bits are used.
    #[inline]
    pub fn stencil_read_mask(mut self, mask: u32) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.stencil.read_mask = mask;
        self
    }

    /// Stencil values are AND'd with this mask when writing to the stencil buffer.
    /// Only low 8 bits are used.
    #[inline]
    pub fn stencil_write_mask(mut self, mask: u32) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.stencil.write_mask = mask;
        self
    }

    /// Specify the full set of depth bias parameters.
    ///
    /// Describes the biasing setting for the depth target.
    #[inline]
    pub fn depth_bias(mut self, bias: wgpu::DepthBiasState) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.bias = bias;
        self
    }

    /// Constant depth biasing factor, in basic units of the depth format.
    #[inline]
    pub fn depth_bias_constant(mut self, constant: i32) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.bias.constant = constant;
        self
    }

    /// Slope depth biasing factor.
    #[inline]
    pub fn depth_bias_slope_scale(mut self, scale: f32) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.bias.slope_scale = scale;
        self
    }

    /// Depth bias clamp value (absolute).
    #[inline]
    pub fn depth_bias_clamp(mut self, clamp: f32) -> Self {
        let state = self
            .depth_stencil
            .get_or_insert(Self::DEFAULT_DEPTH_STENCIL);
        state.bias.clamp = clamp;
        self
    }

    /// If enabled polygon depth is clamped to 0-1 range instead of being clipped.
    ///
    /// Requires `Features::DEPTH_CLIP_CONTROL` enabled.
    #[inline]
    pub fn unclipped_depth(mut self, b: bool) -> Self {
        self.primitive.unclipped_depth = b;
        self
    }

    /// Specify the full multisample state.
    #[inline]
    pub fn multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    /// The number of samples calculated per pixel (for MSAA).
    ///
    /// For non-multisampled textures, this should be 1 (the default).
    #[inline]
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.multisample.count = sample_count;
        self
    }

    /// Bitmask that restricts the samples of a pixel modified by this pipeline. All samples can be
    /// enabled using the value !0 (the default).
    #[inline]
    pub fn sample_mask(mut self, sample_mask: u64) -> Self {
        self.multisample.mask = sample_mask;
        self
    }

    /// When enabled, produces another sample mask per pixel based on the alpha output value, that
    /// is ANDed with the sample_mask and the primitive coverage to restrict the set of samples
    /// affected by a primitive.
    ///
    /// The implicit mask produced for alpha of zero is guaranteed to be zero, and for alpha of one
    /// is guaranteed to be all 1-s.
    ///
    /// Disabled by default.
    #[inline]
    pub fn alpha_to_coverage_enabled(mut self, b: bool) -> Self {
        self.multisample.alpha_to_coverage_enabled = b;
        self
    }

    /// Build the render pipeline layout, its descriptor and ultimately the pipeline itself with
    /// the specified parameters.
    ///
    /// **Panic!**s in the following occur:
    ///
    /// - A rasterization state field was specified but no fragment shader was given.
    /// - A color state field was specified but no fragment shader was given.
    pub fn build(self, device: &'a wgpu::Device) -> wgpu::RenderPipeline {
        let RenderPipelineBuilder {
            layout,
            vertex,
            fragment,
            primitive,
            depth_stencil,
            multisample,
            ..
        } = self;

        let pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&layout),
            vertex,
            primitive,
            depth_stencil,
            multisample,
            fragment,
            multiview: None,
            cache: None,
        };

        device.create_render_pipeline(&pipeline_desc)
    }
}
