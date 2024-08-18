#[derive(Default, Clone, Debug)]
pub struct PipelineLayoutBuilder<'a> {
    label: wgpu::Label<'a>,
    bindings: Vec<&'a wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
}

impl<'a> PipelineLayoutBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn label(mut self, label: impl Into<wgpu::Label<'a>>) -> Self {
        self.label = label.into();
        self
    }

    #[inline]
    pub fn add_binding(mut self, binding: &'a wgpu::BindGroupLayout) -> Self {
        self.bindings.push(binding);
        self
    }

    #[inline]
    pub fn add_push_constant_range(mut self, range: wgpu::PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    #[inline]
    pub fn build(&'a self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self.label,
            bind_group_layouts: &self.bindings,
            push_constant_ranges: &self.push_constant_ranges,
        })
    }
}
