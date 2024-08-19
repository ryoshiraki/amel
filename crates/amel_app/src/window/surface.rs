use amel_gpu::prelude::*;
use std::sync::Arc;

pub struct SurfaceWrapper {
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
    surface_texture: Option<wgpu::SurfaceTexture>,
    depth_texture: Option<Texture>,
}

impl SurfaceWrapper {
    pub fn new() -> Self {
        Self {
            surface: None,
            config: None,
            surface_texture: None,
            depth_texture: None,
        }
    }
    // pub fn new(
    //     window: &Window,
    //     instance: &wgpu::Instance,
    //     adapter: &wgpu::Adapter,
    //     device: &wgpu::Device,
    // ) -> Result<Self, wgpu::CreateSurfaceError> {
    //     let config = window.config();
       
    //     let window = window.window_arc().clone();
    //     let size = window.inner_size();

    //     if cfg!(target_arch = "wasm32") {
    //         self.surface = Some(instance.create_surface(window).unwrap());
    //     }

    //     let surface = instance.create_surface(window)?;
    //     let surface_config = surface
    //         .get_default_config(adapter, size.width, size.height)
    //         .unwrap();
    //     surface.configure(device, &surface_config);

    //     let depth_texture = config.depth_format.map(|format| {
    //         TextureBuilder::new()
    //             .label("depth_texture")
    //             .size(config.size.width, config.size.height)
    //             .format(format)
    //             .sample_count(1)
    //             .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
    //             .build(device)
    //     });

    //     Ok(Self {
    //         surface,
    //         surface_config,
    //         surface_texture: None,
    //         depth_texture,
    //     })
    // }    

    /// Called after the instance is created, but before we request an adapter.
    ///
    /// On wasm, we need to create the surface here, as the WebGL backend needs
    /// a surface (and hence a canvas) to be present to create the adapter.
    ///
    /// We cannot unconditionally create a surface here, as Android requires
    /// us to wait until we receive the `Resumed` event to do so.
    pub fn pre_adapter(&mut self, instance: &wgpu::Instance, window: Arc<winit::window::Window>) {
        if cfg!(target_arch = "wasm32") {
            self.surface = Some(instance.create_surface(window).unwrap());
        }
    }

    /// Check if the event is the start condition for the surface.
    pub fn start_condition(e: &winit::event::Event<()>) -> bool {
        match e {
            // On all other platforms, we can create the surface immediately.
            winit::event::Event::NewEvents(winit::event::StartCause::Init) => !cfg!(target_os = "android"),
            // On android we need to wait for a resumed event to create the surface.
            winit::event::Event::Resumed => cfg!(target_os = "android"),
            _ => false,
        }
    }

    /// Called when an event which matches [`Self::start_condition`] is received.
    ///
    /// On all native platforms, this is where we create the surface.
    ///
    /// Additionally, we configure the surface based on the (now valid) window size.
    pub fn resume(&mut self, instance: &wgpu::Instance, adapter: &wgpu::Adapter, device: &wgpu::Device, window: Arc<winit::window::Window>, srgb: bool) {
        // Window size is only actually valid after we enter the event loop.
        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);

        log::info!("Surface resume {window_size:?}");

        // We didn't create the surface in pre_adapter, so we need to do so now.
        if !cfg!(target_arch = "wasm32") {
            self.surface = Some(instance.create_surface(window).unwrap());
        }

        // From here on, self.surface should be Some.

        let surface = self.surface.as_ref().unwrap();

        // Get the default configuration,
        let mut config = surface
            .get_default_config(&adapter, width, height)
            .expect("Surface isn't supported by the adapter.");
        if srgb {
            // Not all platforms (WebGPU) support sRGB swapchains, so we need to use view formats
            let view_format = config.format.add_srgb_suffix();
            config.view_formats.push(view_format);
        } else {
            // All platforms support non-sRGB swapchains, so we can just use the format directly.
            let format = config.format.remove_srgb_suffix();
            config.format = format;
            config.view_formats.push(format);
        };

        surface.configure(&device, &config);
        self.config = Some(config);
    }

    /// Resize the surface, making sure to not resize to zero.
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let config = self.config.as_mut().unwrap();
        config.width = width.max(1);
        config.height = height.max(1);
        let surface = self.surface.as_ref().unwrap();
        surface.configure(&device, config);
    }

    pub fn acquire(&mut self, device: &wgpu::Device) -> wgpu::SurfaceTexture {
        let surface = self.surface.as_ref().unwrap();

        match surface.get_current_texture() {
            Ok(frame) => frame,
            // If we timed out, just try again
            Err(wgpu::SurfaceError::Timeout) => surface
                .get_current_texture()
                .expect("Failed to acquire next surface texture!"),
            Err(
                // If the surface is outdated, or was lost, reconfigure it.
                wgpu::SurfaceError::Outdated
                | wgpu::SurfaceError::Lost
                // If OutOfMemory happens, reconfiguring may not help, but we might as well try
                | wgpu::SurfaceError::OutOfMemory,
            ) => {
                surface.configure(device, self.config());
                surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        }

    }
    /// On suspend on android, we drop the surface, as it's no longer valid.
    ///
    /// A suspend event is always followed by at least one resume event.
    pub fn suspend(&mut self) {
        if cfg!(target_os = "android") {
            self.surface = None;
        }
    }

    pub fn get(&self) -> Option<&wgpu::Surface> {
        self.surface.as_ref()
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        self.config.as_ref().unwrap()
    }

}
