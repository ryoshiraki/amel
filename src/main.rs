use amel_app::prelude::*;
use amel_gpu::prelude::*;

struct MyApp {
}

#[allow(unused)]
impl App for MyApp {
    fn create(device: &DeviceContext) -> Self {
       MyApp {}
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
