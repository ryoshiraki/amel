use super::config::*;
use super::window::Window;

use amel_gpu::prelude::*;

use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to create surface: {0}")]
    SurfaceCreationError(#[from] wgpu::CreateSurfaceError),
    #[error("Failed to request adapter")]
    AdapterRequestError,
    #[error("Failed to request device: {0}")]
    DeviceRequestError(#[from] wgpu::RequestDeviceError),
}

#[derive(Debug)]
pub struct DeviceContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl DeviceContext {
    pub async fn new(config: DeviceConfig, window: Option<&Window>) -> Result<Self, DeviceError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: config.backends,
            flags: config.flags,
            dx12_shader_compiler: config.dx12_shader_compiler,
            gles_minor_version: config.gles_minor_version,
        });

        let surface = if let Some(window) = window {
            Some(instance.create_surface(window.window())?)
        } else {
            None
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: config.power_preference,
                compatible_surface: surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(DeviceError::AdapterRequestError)?;

        // let adapter = wgpu::util::initialize_adapter_from_env(&instance, surface.as_ref())
        //     .ok_or(DeviceError::AdapterRequestError)?;

        let adapter_info = adapter.get_info();
        log::warn!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let limits = adapter.limits();
        log::warn!("Max push constant size: {}", limits.max_push_constant_size);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: config.features,
                    required_limits: config.limits.clone().using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await?;

        log::warn!(
            "Max push constant size!!!!: {}",
            device.limits().max_push_constant_size
        );
        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
        })
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn device_arc(&self) -> Arc<wgpu::Device> {
        self.device.clone()
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn queue_arc(&self) -> Arc<wgpu::Queue> {
        self.queue.clone()
    }

    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    pub fn submit(&self, command_buffer: wgpu::CommandBuffer) {
        self.queue.submit(std::iter::once(command_buffer));
    }
}
