use super::render_resources::*;
use amel_gpu::prelude::*;
use amel_math::prelude::*;
use amel_mesh::prelude::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub ortho: [f32; 16],
    pub transform: [f32; 16],
    pub color: [f32; 4],
}

struct State {
    background_color: Vec4,
    matrix_stack: MatrixStack,
    color: Vec4,
    orho: Mat4,
}

impl Default for State {
    fn default() -> Self {
        State {
            background_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            matrix_stack: MatrixStack::new(),
            color: Vec4::ONE,
            orho: Mat4::IDENTITY,
        }
    }
}

pub struct RenderContext<'a> {
    render_encoder: &'a mut wgpu::RenderPass<'a>,
    state: State,
    uniform_buffer: DynamicUniformBuffer,
}

impl<'a> RenderContext<'a> {
    pub fn new(device: &wgpu::Device, render_encoder: &'a mut wgpu::RenderPass<'a>) -> Self {
        let uniform_buffer: DynamicUniformBuffer = DynamicUniformBuffer::new::<Uniforms>(
            device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            std::mem::size_of::<Uniforms>(),
            256,
        );

        RenderContext {
            render_encoder,
            state: State::default(),
            uniform_buffer,
        }
    }

    pub fn background(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.state.background_color = Vec4::new(r, g, b, a);
        self
    }

    pub fn color(&mut self, color: Vec4) -> &mut Self {
        self.state.color = color;
        self
    }

    pub fn ortho(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> &mut Self {
        self.state.orho = Mat4::orthographic_lh(left, right, bottom, top, near, far);
        self
    }

    pub fn push_matrix(&mut self) -> &mut Self {
        self.state.matrix_stack.push();
        self
    }

    pub fn pop_matrix(&mut self) -> &mut Self {
        self.state.matrix_stack.pop();
        self
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.state.matrix_stack.translate(translation);
        self
    }

    pub fn scale(&mut self, scale: Vec3) -> &mut Self {
        self.state.matrix_stack.scale(scale);
        self
    }

    pub fn draw_mesh(&mut self, mesh: &GpuMesh) -> &mut Self {
        // let bundle = if let Some(texture) = self.state.borrow().texture {
        //     self.bundle_mesh(mesh, Some(texture))
        // } else {
        //     self.bundle_mesh(mesh, None)
        // };
        // self.render_bundles.push(bundle);
        self.render_encoder.set_bind_group(0, bind_group, &[]);
        self.render_encoder.draw_mesh(mesh);
        self
    }

    pub fn draw_circle(&mut self, radius: f32) -> &mut Self {
        let resources = RENDER_RESOURCES.get().unwrap().lock().unwrap();
        self.push_matrix();
        self.scale(Vec3::new(radius, radius, 1.0));
        self.draw_mesh(&resources.circle().clone());
        self.pop_matrix();
        self
    }

    pub fn draw_rectangle(&mut self, width: f32, height: f32) -> &mut Self {
        let resources = RENDER_RESOURCES.get().unwrap().lock().unwrap();
        self.push_matrix();
        self.scale(Vec3::new(width, height, 1.0));
        self.draw_mesh(&resources.rectangle().clone());
        self.pop_matrix();
        self
    }

    // fn bundle_mesh(&mut self, mesh: &'a GpuMesh) {
    //     // self.render_bundle_encoder.set_bind_group(0, &mesh.bind_group, &[]);
    //     self.render_bundle_encoder.draw_mesh(mesh);
    // }

    // pub fn finish(self) -> Vec<wgpu::RenderBundle> {
    //     self.render_bundles
    // }
}

// use super::render_resources::*;
// use amel_gpu::prelude::*;
// use amel_math::prelude::*;
// use amel_mesh::prelude::*;
// use wgpu::util::RenderEncoder;

// struct State {
//     background_color: Vec4,
//     matrix_stack: MatrixStack,
//     color: Vec4,
//     orho: Mat4,
// }

// impl<'a> Default for State {
//     fn default() -> Self {
//         State {
//             background_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
//             matrix_stack: MatrixStack::new(),
//             color: Vec4::ONE,
//             orho: Mat4::IDENTITY,
//         }
//     }
// }

// pub struct RenderContext<'a, T>
// where
//     T: RenderEncoder<'a> + DrawMesh<'a>,
// {
//     render_encoder: &'a mut T,
//     state: State,
//     render_bundles: Vec<wgpu::RenderBundle>,
// }

// impl<'a, T> RenderContext<'a, T>
// where
//     T: RenderEncoder<'a> + DrawMesh<'a>,
// {
//     pub fn new(render_encoder: &'a mut T) -> Self {
//         RenderContext {
//             render_encoder,
//             state: State::default(),
//             render_bundles: Vec::new(),
//         }
//     }

//     pub fn background(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
//         self.state.background_color = Vec4::new(r, g, b, a);
//         self
//     }

//     pub fn color(&mut self, color: Vec4) -> &mut Self {
//         self.state.color = color;
//         self
//     }

//     pub fn ortho(
//         &mut self,
//         left: f32,
//         right: f32,
//         bottom: f32,
//         top: f32,
//         near: f32,
//         far: f32,
//     ) -> &mut Self {
//         self.state.orho = Mat4::orthographic_lh(left, right, bottom, top, near, far);
//         self
//     }

//     pub fn push_matrix(&mut self) -> &mut Self {
//         self.state.matrix_stack.push();
//         self
//     }

//     pub fn pop_matrix(&mut self) -> &mut Self {
//         self.state.matrix_stack.pop();
//         self
//     }

//     pub fn translate(&mut self, translation: Vec3) -> &mut Self {
//         self.state.matrix_stack.translate(translation);
//         self
//     }

//     pub fn scale(&mut self, scale: Vec3) -> &mut Self {
//         self.state.matrix_stack.scale(scale);
//         self
//     }

//     pub fn draw_mesh(&mut self, mesh: &GpuMesh) -> &mut Self {
//         // let bundle = if let Some(texture) = self.state.borrow().texture {
//         //     self.bundle_mesh(mesh, Some(texture))
//         // } else {
//         //     self.bundle_mesh(mesh, None)
//         // };
//         // self.render_bundles.push(bundle);
//         self.render_encoder.draw_mesh(mesh);
//         self
//     }

//     pub fn draw_circle(&mut self, radius: f32) -> &mut Self {
//         let resources = RENDER_RESOURCES.get().unwrap().lock().unwrap();
//         self.push_matrix();
//         self.scale(Vec3::new(radius, radius, 1.0));
//         self.draw_mesh(&resources.circle().clone());
//         self.pop_matrix();
//         self
//     }

//     pub fn draw_rectangle(&mut self, width: f32, height: f32) -> &mut Self {
//         let resources = RENDER_RESOURCES.get().unwrap().lock().unwrap();
//         self.push_matrix();
//         self.scale(Vec3::new(width, height, 1.0));
//         self.draw_mesh(&resources.rectangle().clone());
//         self.pop_matrix();
//         self
//     }

//     // fn bundle_mesh(&mut self, mesh: &'a GpuMesh) {
//     //     // self.render_bundle_encoder.set_bind_group(0, &mesh.bind_group, &[]);
//     //     self.render_bundle_encoder.draw_mesh(mesh);
//     // }

//     // pub fn finish(self) -> Vec<wgpu::RenderBundle> {
//     //     self.render_bundles
//     // }
// }
