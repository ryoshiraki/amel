use super::render_resources::*;
use amel_gpu::prelude::*;
use amel_math::prelude::*;
use amel_mesh::prelude::*;

struct State {
    background_color: Vec4,
    matrix_stack: MatrixStack,
    color: Vec4,
    orho: Mat4,
}

impl<'a> Default for State {
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
    // render_bundle_encoder: Rc<RefCell<wgpu::RenderBundleEncoder<'static>>>,
    render_bundle_encoder: wgpu::RenderBundleEncoder<'a>,
    state: State,
    render_bundles: Vec<wgpu::RenderBundle>,
}

impl<'a> RenderContext<'a> {
    pub fn new(render_bundle_encoder: wgpu::RenderBundleEncoder<'a>) -> Self {
        RenderContext {
            render_bundle_encoder,
            state: State::default(),
            render_bundles: Vec::new(),
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

    fn bundle_mesh(&mut self, mesh: &'a GpuMesh) {
        // self.render_bundle_encoder.set_bind_group(0, &mesh.bind_group, &[]);
        self.render_bundle_encoder.draw_mesh(mesh);
    }

    pub fn finish(self) -> Vec<wgpu::RenderBundle> {
        self.render_bundles
    }
}
