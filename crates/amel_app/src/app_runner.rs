use std::collections::HashMap;

use thiserror::Error;
use cfg_if::cfg_if;

use super::config::*;
use super::device::*;
use super::window::Window;
use amel_math::prelude::*;

#[allow(unused)]
pub trait App: Sized {
    fn create(device: &DeviceContext) -> Self;
    // fn update(&mut self, device: &DeviceContext, window: &Window);
    // fn render(&mut self, draw: &mut Renderer, window: &Window);
    // fn resize(&mut self, width: u32, height: u32) {}
    // fn key_down(&mut self, key: &str) {}
    // fn key_up(&mut self, key: &str) {}
    // fn cursor_moved(&mut self, position: Vec2) {}
    // fn mouse_down(&mut self, position: Vec2) {}
    // fn mouse_up(&mut self, position: Vec2) {}
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Initialization failed: {0}")]
    Initialization(String),
    #[error("Window creation failed")]
    WindowCreation(#[from] winit::error::OsError),
    #[error("Surface creation failed")]
    SurfaceCreation(String),
    #[error("Gpu initialization failed")]
    GpuInitialization(String),
    #[error("Event loop failed")]
    EventLoop(String),
}

pub struct AppRunner {}

impl AppRunner {
    async fn start<A: App>(config: AppConfig) -> Result<(), AppError> {
        let event_loop = winit::event_loop::EventLoop::new().map_err(|e| AppError::Initialization(e.to_string()))?;

        let mut windows = HashMap::new();
        for window_config in config.window_configs {
            let window = Window::new(window_config, &event_loop)?;
            windows.insert(window.id(), window);
        }

        let device_context = DeviceContext::new(config.device_config, windows.values().next())
        .await
        .map_err(|e| AppError::GpuInitialization(e.to_string()))?;

        for window in windows.values() {
        }

        let mut app = None;

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                use winit::platform::web::EventLoopExtWebSys;
                let event_loop_function = winit::event_loop::EventLoop::spawn;
            } else {
                let event_loop_function = winit::event_loop::EventLoop::run;
            }
        }

        let _ = (event_loop_function)(
            event_loop,
            move |event: winit::event::Event<()>, target: &winit::event_loop::EventLoopWindowTarget<()>| {
                match event {
                    ref e if start_condition(e) => {
                        //surface.resume(&context, window_loop.window.clone(), A::SRGB);
                        if app.is_none() {
                            app = Some(A::create(&device_context));
                        }
                    },
                    winit::event::Event::Suspended => {
                        // surface.suspend();
                    },
                    winit::event::Event::WindowEvent { event, window_id } => {
                        if let Some(window) = windows.get_mut(&window_id) {
                            match event {
                                winit::event::WindowEvent::Resized(size) => {
                                    // window.resize(size.width, size.height);
                                    // app.resize(size.width, size.height);
                                    window.request_redraw();
                                },
                                winit::event::WindowEvent::RedrawRequested => {
                                    if app.is_none() {
                                        return;
                                    }

                                    let app = app.as_mut().unwrap();


                                    window.request_redraw();
                                }
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            }
        );

        Ok(())
    }

    pub fn run<A: App>(config: AppConfig) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                if let Err(e) =  wasm_bindgen_futures::spawn_local(async move { start::<E>(title).await }) {
                    log::error!("{}", e);
                }
            } else {
                if let Err(e) =  pollster::block_on(Self::start::<A>(config)) {
                    log::error!("{}", e);
                }
            }
        }    
    }
}

fn start_condition(e: &winit::event::Event<()>) -> bool {
    match e {
        // On all other platforms, we can create the surface immediately.
        winit::event::Event::NewEvents(winit::event::StartCause::Init) => !cfg!(target_os = "android"),
        // On android we need to wait for a resumed event to create the surface.
        winit::event::Event::Resumed => cfg!(target_os = "android"),
        _ => false,
    }
}