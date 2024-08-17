use amel_gpu::prelude::*;

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub backends: wgpu::Backends,
    pub flags: wgpu::InstanceFlags,
    pub dx12_shader_compiler: wgpu::Dx12Compiler,
    pub gles_minor_version: wgpu::Gles3MinorVersion,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub power_preference: wgpu::PowerPreference,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            backends: wgpu::util::backend_bits_from_env().unwrap_or_default(),
            flags: wgpu::InstanceFlags::from_build_config().with_env(),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
            gles_minor_version: wgpu::util::gles_minor_version_from_env().unwrap_or_default(),
            features: wgpu::Features::empty(),
            limits: wgpu::Limits {
                max_push_constant_size: 256, // Set this to an appropriate value
                ..wgpu::Limits::default()
            },
            power_preference: wgpu::PowerPreference::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeviceConfigBuilder {
    config: DeviceConfig,
}

impl DeviceConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn high_performance() -> Self {
        Self {
            config: DeviceConfig {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            },
        }
    }

    pub fn with_power_preference(&mut self, power_preference: wgpu::PowerPreference) -> &mut Self {
        self.config.power_preference = power_preference;
        self
    }

    pub fn with_features(&mut self, features: wgpu::Features) -> &mut Self {
        self.config.features = features;
        self
    }

    pub fn with_limits(&mut self, limits: wgpu::Limits) -> &mut Self {
        self.config.limits = limits;
        self
    }

    pub fn with_backends(&mut self, backends: wgpu::Backends) -> &mut Self {
        self.config.backends = backends;
        self
    }

    pub fn with_instance_flags(&mut self, flags: wgpu::InstanceFlags) -> &mut Self {
        self.config.flags = flags;
        self
    }

    pub fn build(&self) -> DeviceConfig {
        self.config.clone()
    }
}
