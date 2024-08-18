use std::num::NonZeroU64;

#[derive(Debug)]
pub enum BindingType {
    StorageBuffer {
        read_only: bool,
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZeroU64>,
    },
    // StorageBufferSized {
    //     read_only: bool,
    //     has_dynamic_offset: bool,
    // },
    UniformBuffer {
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZeroU64>,
    },
    // UniformBufferSized {
    //     has_dynamic_offset: bool,
    // },
    Texture1D {
        filterable: bool,
    },
    Texture2D {
        filterable: bool,
        multisampled: bool,
    },
    Texture3D {
        filterable: bool,
        multisampled: bool,
    },
    TextureDepth2D {
        multisampled: bool,
    },
    Texture2DArray {
        filterable: bool,
        multisampled: bool,
    },
    Sampler {
        filterable: bool,
    },
    ComparisonSampler,
}

impl BindingType {
    pub fn to_wgpu(&self) -> wgpu::BindingType {
        match self {
            BindingType::StorageBuffer {
                read_only,
                has_dynamic_offset,
                min_binding_size,
            } => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: *read_only,
                },
                has_dynamic_offset: *has_dynamic_offset,
                min_binding_size: *min_binding_size,
            },
            BindingType::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: *has_dynamic_offset,
                min_binding_size: *min_binding_size,
            },
            BindingType::Texture1D { filterable } => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float {
                    filterable: *filterable,
                },
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D1,
            },
            BindingType::Texture2D {
                filterable,
                multisampled,
            } => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float {
                    filterable: if *multisampled { false } else { *filterable },
                },
                multisampled: *multisampled,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            BindingType::Texture3D {
                filterable,
                multisampled,
            } => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float {
                    filterable: if *multisampled { false } else { *filterable },
                },
                multisampled: *multisampled,
                view_dimension: wgpu::TextureViewDimension::D3,
            },
            BindingType::TextureDepth2D { multisampled } => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Depth,
                multisampled: *multisampled,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            BindingType::Texture2DArray {
                filterable,
                multisampled,
            } => wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float {
                    filterable: if *multisampled { false } else { *filterable },
                },
                multisampled: *multisampled,
                view_dimension: wgpu::TextureViewDimension::D2Array,
            },
            BindingType::Sampler { filterable } => wgpu::BindingType::Sampler(if *filterable {
                wgpu::SamplerBindingType::Filtering
            } else {
                wgpu::SamplerBindingType::NonFiltering
            }),
            BindingType::ComparisonSampler => {
                wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison)
            }
        }
    }
}

#[derive(Debug)]
pub struct Binding {
    pub binding: BindingType,
    pub stage: wgpu::ShaderStages,
}

#[derive(Debug, Default)]
pub struct BindGroupLayoutBuilder {
    bindings: Vec<Binding>,
}

impl BindGroupLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, binding_type: BindingType, stage: wgpu::ShaderStages) -> Self {
        self.bindings.push(Binding {
            binding: binding_type,
            stage,
        });
        self
    }

    pub fn add_to_vertex_stage(self, binding_type: BindingType) -> Self {
        self.add(binding_type, wgpu::ShaderStages::VERTEX)
    }

    pub fn add_to_fragment_stage(self, binding_type: BindingType) -> Self {
        self.add(binding_type, wgpu::ShaderStages::FRAGMENT)
    }

    pub fn add_to_compute_stage(self, binding_type: BindingType) -> Self {
        self.add(binding_type, wgpu::ShaderStages::COMPUTE)
    }

    pub fn add_to_vertex_fragment_stage(self, binding_type: BindingType) -> Self {
        self.add(binding_type, wgpu::ShaderStages::VERTEX_FRAGMENT)
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = self
            .bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: binding.stage,
                ty: binding.binding.to_wgpu(),
                count: None,
            })
            .collect();

        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &entries,
            label: None,
        })
    }
}
