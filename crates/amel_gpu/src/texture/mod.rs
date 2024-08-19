use bytemuck::Pod;
use std::{ops::Deref, sync::Arc};

use self::prelude::{SamplerBuilder, TextureViewBuilder};

pub mod sampler_builder;
pub mod texture_builder;
pub mod texture_view_builder;

pub mod prelude {
    pub use super::sampler_builder::*;
    pub use super::texture_builder::*;
    pub use super::texture_view_builder::*;
    pub use super::Texture;
}

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: Arc<wgpu::TextureView>, //to do: change to Vec[wgpu::TextureView] for mip_count
    pub sampler: Arc<wgpu::Sampler>,
}

impl Texture {
    pub fn new(device: &wgpu::Device, desc: &wgpu::TextureDescriptor) -> Self {
        let texture = device.create_texture(desc);

        let view = Arc::new(TextureViewBuilder::new().build(&texture));
        let sampler = Arc::new(SamplerBuilder::new().build(device));

        // let mut views = vec![];
        // for mip_level in 0..mip_count {
        //     let view = texture.create_view(&TextureViewDescriptor {
        //         base_mip_level: mip_level,
        //         mip_level_count: Some(1),
        //         ..Default::default()
        //     });
        //     views.push(view);
        // }

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn new_with_data<T: Pod>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        desc: &wgpu::TextureDescriptor,
        data: &[T],
    ) -> Self {
        let texture = device.create_texture(desc);
        let view = Arc::new(TextureViewBuilder::new().build(&texture));
        let sampler = Arc::new(SamplerBuilder::new().build(device));

        let texture_obj = Self {
            texture,
            view,
            sampler,
        };

        texture_obj.write_texture_data(queue, data, desc.size);

        texture_obj
    }

    fn write_texture_data<T: Pod>(&self, queue: &wgpu::Queue, data: &[T], size: wgpu::Extent3d) {
        let bytes_per_pixel = 4; //std::mem::size_of::<T>() as u32;
        let bytes_per_row = bytes_per_pixel * size.width; //bytes_per_pixel * size.width;

        let data: &[u8] = bytemuck::cast_slice(data);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(size.height),
            },
            size,
        );
    }

    pub fn size_bytes<T: Pod>(&self) -> usize {
        let size = self.texture.size();
        (size.width * size.height) as usize * std::mem::size_of::<T>()
    }

    pub fn width(&self) -> u32 {
        self.texture.size().width
    }

    pub fn height(&self) -> u32 {
        self.texture.size().height
    }

    pub fn depth(&self) -> u32 {
        self.texture.size().depth_or_array_layers
    }

    pub fn fill<T: Pod>(&self, queue: &wgpu::Queue, data: &[T])
    where
        T: Clone + Copy,
    {
        let size = self.texture.size();
        self.write_texture_data(queue, data, size);
    }

    pub fn clear<T: Pod>(&self, queue: &wgpu::Queue, value: T) {
        let size = self.texture.size();
        let capacity = (size.width * size.height * size.depth_or_array_layers) as usize;
        let texels = vec![value; capacity];
        self.fill(queue, &texels);
    }
}

impl Deref for Texture {
    type Target = wgpu::Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

// impl From<Framebuffer> for Texture {
//     fn from(fb: Framebuffer) -> Self {
//         fb.texture
//     }
// }

// use std::ops::Deref;

// use bytemuck::Pod;
// use wgpu::util::DeviceExt;

// use self::prelude::{SamplerBuilder, TextureViewBuilder};

// pub mod sampler_builder;
// pub mod texture_builder;
// pub mod texture_view_builder;

// pub mod prelude {
//     pub use super::sampler_builder::*;
//     pub use super::texture_builder::*;
//     pub use super::texture_view_builder::*;
//     pub use super::Texture;
// }

// #[derive(Debug)]
// pub struct Texture {
//     pub texture: wgpu::Texture,
//     pub view: wgpu::TextureView, //to do: change to Vec[wgpu::TextureView] for mip_count
//     pub sampler: wgpu::Sampler,
// }

// impl Texture {
//     pub fn new(device: &wgpu::Device, desc: &wgpu::TextureDescriptor) -> Self {
//         let texture = device.create_texture(desc);

//         let view = TextureViewBuilder::new().build(&texture);
//         let sampler = SamplerBuilder::new().build(device);

//         // let mut views = vec![];
//         // for mip_level in 0..mip_count {
//         //     let view = texture.create_view(&TextureViewDescriptor {
//         //         base_mip_level: mip_level,
//         //         mip_level_count: Some(1),
//         //         ..Default::default()
//         //     });
//         //     views.push(view);
//         // }

//         Self {
//             texture,
//             view,
//             sampler,
//         }
//     }

//     pub fn new_with_data<T: Pod>(
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         desc: &wgpu::TextureDescriptor,
//         data: &[T],
//     ) {
//         let texture = device.create_texture(desc);
//         let view = TextureViewBuilder::new().build(&texture);
//         let sampler = SamplerBuilder::new().build(device);

//     }

//     pub fn size_bytes<T: Pod>(&self) -> usize {
//         let size = self.texture.size();
//         (size.width * size.height) as usize * std::mem::size_of::<T>()
//     }

//     pub fn width(&self) -> u32 {
//         self.texture.size().width
//     }

//     pub fn height(&self) -> u32 {
//         self.texture.size().height
//     }

//     pub fn depth(&self) -> u32 {
//         self.texture.size().depth_or_array_layers
//     }

//     pub fn copy<T: Pod>(
//         &self,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         data: &[T],
//         origin: wgpu::Origin3d,
//         size: wgpu::Extent3d,
//     ) where
//         T: Clone + Copy,
//     {
//         let bytes_per_pixel = 4; // Assuming the data is RGBA (4 bytes per pixel)
//         let expected_size =
//             (size.width * size.height * size.depth_or_array_layers) as usize * bytes_per_pixel;

//         assert_eq!(
//             std::mem::size_of_val(data),
//             expected_size,
//             "Data size does not match the specified region size."
//         );

//         let data: &[u8] = bytemuck::cast_slice(data);
//         let bytes_per_row = bytes_per_pixel as u32 * size.width;
//         let rows_per_image = size.height;

//         let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Staging Buffer"),
//             contents: data,
//             usage: wgpu::BufferUsages::COPY_SRC,
//         });

//         let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//             label: Some("Texture Upload Encoder"),
//         });

//         encoder.copy_buffer_to_texture(
//             wgpu::ImageCopyBuffer {
//                 buffer: &buffer,
//                 layout: wgpu::ImageDataLayout {
//                     offset: 0,
//                     bytes_per_row: Some(bytes_per_row),
//                     rows_per_image: Some(rows_per_image),
//                 },
//             },
//             wgpu::ImageCopyTexture {
//                 texture: &self.texture,
//                 mip_level: 0,
//                 origin,
//                 aspect: wgpu::TextureAspect::All,
//             },
//             size,
//         );

//         queue.submit(Some(encoder.finish()));
//     }

//     pub fn fill<T: Pod>(&self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T])
//     where
//         T: Clone + Copy,
//     {
//         let size = self.texture.size();
//         self.copy(
//             device,
//             queue,
//             data,
//             wgpu::Origin3d { x: 0, y: 0, z: 0 },
//             size,
//         );
//     }

//     pub fn clear<T: Pod>(&self, device: &wgpu::Device, queue: &wgpu::Queue, value: T) {
//         let size = self.texture.size();
//         let capacity = (size.width * size.height * size.depth_or_array_layers) as usize;
//         let texels = vec![value; capacity];
//         self.fill(device, queue, &texels);
//     }
// }

// impl Deref for Texture {
//     type Target = wgpu::Texture;

//     fn deref(&self) -> &Self::Target {
//         &self.texture
//     }
// }

// // impl From<Framebuffer> for Texture {
// //     fn from(fb: Framebuffer) -> Self {
// //         fb.texture
// //     }
// // }
