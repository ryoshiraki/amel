#[derive(Debug)]
pub struct FragmentStateBuilder<'a> {
    pub shader: Option<&'a wgpu::ShaderModule>,
    pub entry_point: &'a str,
    pub compilation_options: wgpu::PipelineCompilationOptions<'a>,
    pub targets: Option<&'a [Option<wgpu::ColorTargetState>]>,
}

impl<'a> Default for FragmentStateBuilder<'a> {
    fn default() -> Self {
        FragmentStateBuilder {
            shader: None,
            entry_point: "main",
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: None,
        }
    }
}

impl<'a> FragmentStateBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shader(mut self, shader: &'a wgpu::ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn entry_point(mut self, entry_point: &'a str) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub fn compilation_options(mut self, options: wgpu::PipelineCompilationOptions<'a>) -> Self {
        self.compilation_options = options;
        self
    }

    pub fn targets(mut self, targets: &'a [Option<wgpu::ColorTargetState>]) -> Self {
        self.targets = Some(targets);
        self
    }

    pub fn build(self) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module: self.shader.expect("Shader module is required"),
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            targets: self.targets.unwrap(),
        }
    }
}

pub struct ColorTargetState {
    pub format: wgpu::TextureFormat,
    pub blend: Option<wgpu::BlendState>,
    pub write_mask: wgpu::ColorWrites,
}

impl Default for ColorTargetState {
    fn default() -> Self {
        ColorTargetState {
            format: ColorTargetState::DEFAULT_COLOR_FORMAT,
            blend: Some(ColorTargetState::DEFAULT_BLEND_STATE),
            write_mask: Self::DEFAULT_COLOR_WRITE,
        }
    }
}

impl ColorTargetState {
    pub const DEFAULT_COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb; //wgpu::TextureFormat::Rgba16Float;
    pub const DEFAULT_COLOR_BLEND: wgpu::BlendComponent = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    };
    pub const DEFAULT_ALPHA_BLEND: wgpu::BlendComponent = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    };
    pub const DEFAULT_COLOR_WRITE: wgpu::ColorWrites = wgpu::ColorWrites::ALL;
    pub const DEFAULT_BLEND_STATE: wgpu::BlendState = wgpu::BlendState {
        color: Self::DEFAULT_COLOR_BLEND,
        alpha: Self::DEFAULT_ALPHA_BLEND,
    };
    pub const DEFAULT_COLOR_STATE: wgpu::ColorTargetState = wgpu::ColorTargetState {
        format: Self::DEFAULT_COLOR_FORMAT,
        blend: Some(Self::DEFAULT_BLEND_STATE),
        write_mask: Self::DEFAULT_COLOR_WRITE,
    };

    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn blend(mut self, blend: wgpu::BlendState) -> Self {
        self.blend = Some(blend);
        self
    }

    pub fn color_blend(mut self, color_blend: wgpu::BlendComponent) -> Self {
        let blend = self.blend.get_or_insert(Self::DEFAULT_BLEND_STATE);
        blend.color = color_blend;
        self
    }

    pub fn alpha_blend(mut self, alpha_blend: wgpu::BlendComponent) -> Self {
        let blend = self.blend.get_or_insert(Self::DEFAULT_BLEND_STATE);
        blend.alpha = alpha_blend;
        self
    }

    pub fn write_mask(mut self, write_mask: wgpu::ColorWrites) -> Self {
        self.write_mask = write_mask;
        self
    }
}

#[derive(Default)]
pub struct ColorTargetStatesBuilder {
    color_target_states: Vec<Option<ColorTargetState>>,
}

impl ColorTargetStatesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_target(mut self, target: Option<ColorTargetState>) -> Self {
        self.color_target_states.push(target);
        self
    }

    pub fn build(self) -> Vec<Option<wgpu::ColorTargetState>> {
        self.color_target_states
            .iter()
            .map(|target| {
                target.as_ref().map(|target| wgpu::ColorTargetState {
                    format: target.format,
                    blend: target.blend,
                    write_mask: target.write_mask,
                })
            })
            .collect()
    }
}
