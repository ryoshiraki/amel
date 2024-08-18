pub trait IntoBinding<'a> {
    fn into_binding(self) -> wgpu::BindingResource<'a>;
}

impl<'a> IntoBinding<'a> for &'a wgpu::Sampler {
    #[inline]
    fn into_binding(self) -> wgpu::BindingResource<'a> {
        wgpu::BindingResource::Sampler(self)
    }
}

impl<'a> IntoBinding<'a> for &'a wgpu::TextureView {
    #[inline]
    fn into_binding(self) -> wgpu::BindingResource<'a> {
        wgpu::BindingResource::TextureView(self)
    }
}

impl<'a> IntoBinding<'a> for &'a [&'a wgpu::TextureView] {
    #[inline]
    fn into_binding(self) -> wgpu::BindingResource<'a> {
        wgpu::BindingResource::TextureViewArray(self)
    }
}

impl<'a> IntoBinding<'a> for wgpu::BufferBinding<'a> {
    #[inline]
    fn into_binding(self) -> wgpu::BindingResource<'a> {
        wgpu::BindingResource::Buffer(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BindGroupBuilder<'a> {
    entries: Vec<wgpu::BindingResource<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry<T: IntoBinding<'a>>(mut self, resource: T) -> Self {
        self.entries.push(resource.into_binding());
        self
    }

    pub fn build(self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = self
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, resource)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource,
            })
            .collect();

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &entries,
            label: None,
        })
    }
}
