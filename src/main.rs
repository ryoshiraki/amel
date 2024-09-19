use amel_app::prelude::*;
use amel_gpu::prelude::*;
use amel_math::prelude::*;
use amel_renderer::prelude::*;

struct MyApp {}

#[allow(unused)]
impl App for MyApp {
    fn create(device: &DeviceContext) -> Self {
        MyApp {}
    }

    fn render(&mut self, context: &mut RenderContext, window: &Window) {
        context.ortho(0.0, window.width(), window.height(), 0.0, -1.0, 1.0);

        context
            .color(Vec4::new(1.0, 1.0, 1.0, 1.0))
            .push_matrix()
            .translate(Vec3::new(100.0, 100.0, 0.0))
            .draw_circle(100.0)
            .pop_matrix();
    }
}

pub fn main() {
    let config = AppConfigBuilder::new()
        .with_device_config(
            DeviceConfigBuilder::high_performance()
                .with_features(
                    wgpu::Features::PUSH_CONSTANTS
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                )
                .build(),
        )
        .with_window_config(
            WindowConfigBuilder::new()
                .with_title("Hello, World!")
                .with_size(960, 600)
                .with_position(0, 0)
                .with_depth_format(None)
                .build(),
        )
        .build();

    AppRunner::run::<MyApp>(config);
}
