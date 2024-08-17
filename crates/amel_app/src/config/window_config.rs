use amel_gpu::prelude::*;
use winit::dpi::{LogicalPosition, LogicalSize};

#[derive(Debug, Copy, Clone)]
pub struct WindowConfig {
    pub title: &'static str,
    pub position: Option<LogicalPosition<u32>>,
    pub size: LogicalSize<u32>,
    pub present_mode: wgpu::PresentMode,
    pub alpha_mode: wgpu::CompositeAlphaMode,
    pub surface_format: wgpu::TextureFormat,
    pub depth_format: Option<wgpu::TextureFormat>,
    pub blend_mode: wgpu::BlendState,
    pub desired_maximum_frame_latency: u32,
    pub min_size: Option<LogicalSize<u32>>,
    pub max_size: Option<LogicalSize<u32>>,
    pub resizable: bool,
    pub fullscreen: bool,
    pub sample_count: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Window",
            position: None,
            size: LogicalSize::new(800, 600),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            surface_format: wgpu::TextureFormat::Bgra8UnormSrgb,
            depth_format: None,
            blend_mode: wgpu::BlendState::ALPHA_BLENDING,
            desired_maximum_frame_latency: 2,
            max_size: None,
            min_size: None,
            resizable: true,
            fullscreen: false,
            sample_count: 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WindowConfigBuilder {
    config: WindowConfig,
}

impl WindowConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(&mut self, title: &'static str) -> &mut Self {
        self.config.title = title;
        self
    }

    pub fn with_position(&mut self, x: u32, y: u32) -> &mut Self {
        self.config.position = Some(LogicalPosition::new(x, y));
        self
    }

    pub fn with_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.size = LogicalSize::new(width, height);
        self
    }

    pub fn with_present_mode(&mut self, present_mode: wgpu::PresentMode) -> &mut Self {
        self.config.present_mode = present_mode;
        self
    }

    pub fn with_alpha_mode(&mut self, alpha_mode: wgpu::CompositeAlphaMode) -> &mut Self {
        self.config.alpha_mode = alpha_mode;
        self
    }

    pub fn with_surface_format(&mut self, surface_format: wgpu::TextureFormat) -> &mut Self {
        self.config.surface_format = surface_format;
        self
    }

    pub fn with_depth_format(&mut self, depth_format: Option<wgpu::TextureFormat>) -> &mut Self {
        self.config.depth_format = depth_format;
        self
    }

    pub fn with_sample_count(&mut self, sample_count: u32) -> &mut Self {
        self.config.sample_count = sample_count;
        self
    }

    pub fn with_desired_maximum_frame_latency(
        &mut self,
        desired_maximum_frame_latency: u32,
    ) -> &mut Self {
        self.config.desired_maximum_frame_latency = desired_maximum_frame_latency;
        self
    }

    pub fn with_max_size(&mut self, max_size: LogicalSize<u32>) -> &mut Self {
        self.config.max_size = Some(max_size);
        self
    }

    pub fn with_min_size(&mut self, min_size: LogicalSize<u32>) -> &mut Self {
        self.config.min_size = Some(min_size);
        self
    }

    pub fn with_resizable(&mut self, resizable: bool) -> &mut Self {
        self.config.resizable = resizable;
        self
    }

    pub fn with_fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.config.fullscreen = fullscreen;
        self
    }

    pub fn build(&self) -> WindowConfig {
        self.config
    }
}

// pub struct Window {}
