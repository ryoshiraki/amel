use super::{device_config::DeviceConfig, window_config::WindowConfig};
use amel_gpu::prelude::*;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub color_format: wgpu::TextureFormat,
    pub depth_format: Option<wgpu::TextureFormat>,
    pub sample_count: u32,
    pub device_config: DeviceConfig,
    pub window_configs: Vec<WindowConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            color_format: wgpu::TextureFormat::Bgra8UnormSrgb,
            depth_format: None,
            sample_count: 1,
            device_config: DeviceConfig::default(),
            window_configs: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppConfigBuilder {
    config: AppConfig,
}

impl AppConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_device_config(mut self, device_config: DeviceConfig) -> Self {
        self.config.device_config = device_config;
        self
    }

    pub fn with_window_config(mut self, window_config: WindowConfig) -> Self {
        self.config.window_configs.push(window_config);
        self
    }

    pub fn build(self) -> AppConfig {
        self.config
    }
}
