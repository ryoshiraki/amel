pub mod frame_counter;
pub mod surface;

pub use frame_counter::*;
pub use surface::*;

use super::config::window_config::WindowConfig;
use std::sync::Arc;

use amel_gpu::prelude::*;
use amel_math::prelude::*;

pub struct Window {
    window: Arc<winit::window::Window>,
    config: WindowConfig,
    frame_counter: FrameCounter,
    surface: SurfaceWrapper,
    // surface_render_target: Option<SurfaceRenderTarget>,
    surface_texture: Option<wgpu::SurfaceTexture>,
    depth_texture: Option<Texture>,
}

impl RenderTarget for Window {
    fn color_formats(&self) -> Vec<Option<wgpu::TextureFormat>> {
        vec![Some(self.config.surface_format)]
    }

    fn depth_format(&self) -> Option<wgpu::TextureFormat> {
        self.config.depth_format
    }

    fn color_views(&self) -> Vec<Option<wgpu::TextureView>> {
        vec![self
            .surface_texture
            .as_ref()
            .map(|surface_texture| TextureViewBuilder::new().build(&surface_texture.texture))]
    }

    fn depth_view(&self) -> Option<wgpu::TextureView> {
        self.depth_texture
            .as_ref()
            .map(|depth_texture| depth_texture.view())
    }
}

impl Window {
    pub fn new(
        config: WindowConfig,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Self, winit::error::OsError> {
        let mut builder = winit::window::WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(config.size)
            .with_resizable(config.resizable)
            .with_visible(true)
            .with_window_icon(None);

        if let Some(min_size) = config.min_size {
            builder = builder.with_min_inner_size(min_size);
        }

        if let Some(max_size) = config.max_size {
            builder = builder.with_max_inner_size(max_size);
        }

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowBuilderExtWebSys;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            builder = builder.with_canvas(Some(canvas));
        }

        let window = builder.build(event_loop)?;

        if let Some(position) = config.position {
            window.set_outer_position(position);
        }

        if config.fullscreen {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        }

        Ok(Self {
            window: Arc::new(window),
            config,
            frame_counter: FrameCounter::new(),
            surface: SurfaceWrapper::new(),
            surface_texture: None,
            depth_texture: None,
        })
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn config(&self) -> &WindowConfig {
        &self.config
    }

    pub fn surface(&mut self) -> &mut SurfaceWrapper {
        &mut self.surface
    }

    pub fn pre_adapter(&mut self, instance: &wgpu::Instance) {
        self.surface.pre_adapter(instance, self.window_arc());
    }

    pub fn update(&mut self, device: &wgpu::Device) {
        self.frame_counter.update();
        self.surface_texture = Some(self.surface.acquire(device));
    }

    pub fn present(&mut self) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
    }

    pub fn resume(
        &mut self,
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
    ) {
        self.surface
            .resume(instance, adapter, device, self.window_arc(), false);

        self.depth_texture = self.config.depth_format.map(|format| {
            TextureBuilder::new()
                .label("depth_texture")
                .size(self.config.size.width, self.config.size.height)
                .format(format)
                .sample_count(1)
                .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
                .build(device)
        });
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.config.size = winit::dpi::LogicalSize::new(width, height);
        self.surface.resize(device, width, height);

        if let Some(ref mut depth_texture) = self.depth_texture {
            *depth_texture = TextureBuilder::new()
                .label("depth_texture")
                .size(width, height)
                .format(depth_texture.format())
                .sample_count(1)
                .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
                .build(device);
        }
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.frame_counter.elapsed_secs()
    }

    pub fn fps(&self) -> f32 {
        self.frame_counter.fps()
    }

    pub fn elapsed_frames(&self) -> u32 {
        self.frame_counter.frame_count()
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    /// - **iOS:** Can only be called on the main thread. Returns the top left coordinates of the
    /// window in the screen space coordinate system.
    /// - **Web:** Returns the top-left coordinates relative to the viewport.
    pub fn outer_position_pixels(&self) -> Result<(i32, i32), winit::error::NotSupportedError> {
        self.window.outer_position().map(Into::into)
    }

    /// - **iOS:** Can only be called on the main thread. Sets the top left coordinates of the
    ///   window in the screen space coordinate system.
    /// - **Web:** Sets the top-left coordinates relative to the viewport.
    pub fn set_outer_position_pixels(&self, x: i32, y: i32) {
        self.window
            .set_outer_position(winit::dpi::PhysicalPosition { x, y })
    }

    pub fn inner_size_pixels(&self) -> Vec2 {
        self.window.inner_size().to_vec2()
    }

    pub fn inner_size_points(&self) -> Vec2 {
        self.window
            .inner_size()
            .to_logical::<f64>(self.window.scale_factor())
            .to_vec2()
    }

    pub fn set_inner_size_pixels(&self, width: u32, height: u32) {
        let _ = self
            .window
            .request_inner_size(winit::dpi::PhysicalSize { width, height });
    }

    pub fn set_inner_size_points(&self, width: f32, height: f32) {
        let _ = self
            .window
            .request_inner_size(winit::dpi::LogicalSize { width, height });
    }

    pub fn outer_size_pixels(&self) -> Vec2 {
        self.window.outer_size().to_vec2()
    }

    pub fn outer_size_points(&self) -> Vec2 {
        self.window
            .outer_size()
            .to_logical::<f64>(self.window.scale_factor())
            .to_vec2()
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn window_arc(&self) -> Arc<winit::window::Window> {
        self.window.clone()
    }

    pub fn set_decorations(&self, decorations: bool) {
        self.window.set_decorations(decorations)
    }

    pub fn set_always_on_top(&self, always_on_top: bool) {
        self.window.set_window_level(if always_on_top {
            winit::window::WindowLevel::AlwaysOnTop
        } else {
            winit::window::WindowLevel::Normal
        })
    }

    pub fn width(&self) -> f32 {
        self.inner_size_points().x
    }

    pub fn height(&self) -> f32 {
        self.inner_size_points().y
    }

    // pub fn set_fullscreen(&self, fullscreen: bool) {
    //     if fullscreen {
    //         let monitor = self.current_monitor();
    //         let fullscreen = Fullscreen::Borderless(monitor);
    //         self.set_fullscreen_with(Some(fullscreen));
    //     } else {
    //         self.set_fullscreen_with(None);
    //     }
    // }
}

trait ToVec2 {
    fn to_vec2(&self) -> Vec2;
}

impl ToVec2 for winit::dpi::LogicalSize<f64> {
    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl ToVec2 for winit::dpi::PhysicalSize<u32> {
    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl ToVec2 for winit::dpi::LogicalPosition<f64> {
    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl ToVec2 for winit::dpi::PhysicalPosition<i32> {
    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}
