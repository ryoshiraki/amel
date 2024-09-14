use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct MatrixStack {
    stack: Vec<Mat4>,
}

impl Default for MatrixStack {
    fn default() -> Self {
        Self {
            stack: vec![Mat4::IDENTITY],
        }
    }
}

impl MatrixStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> &Mat4 {
        self.stack.last().expect("Stack is empty")
    }

    pub fn push(&mut self) -> &mut Self {
        let top = *self.get();
        self.stack.push(top);
        self
    }

    pub fn push_matrix(&mut self, mat: &Mat4) -> &mut Self {
        self.stack.push(*mat);
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        self.stack.pop().expect("Stack underflow");
        assert!(!self.stack.is_empty(), "Stack is empty after pop");
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        *self.get_mut() = Mat4::IDENTITY;
        self
    }

    pub fn mult(&mut self, mat: &Mat4) -> &mut Self {
        *self.get_mut() *= *mat;
        self
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.mult(&Mat4::from_translation(translation))
    }

    pub fn rotate(&mut self, theta: f32, axis: Vec3) -> &mut Self {
        self.mult(&Mat4::from_axis_angle(axis.normalize(), theta))
    }

    // rotate_radian is the same as rotate, so it's not explicitly needed

    pub fn rotate_degree(&mut self, angle: f32, axis: Vec3) -> &mut Self {
        self.rotate(angle.to_radians(), axis)
    }

    pub fn rotate_mat3(&mut self, rotation: &Mat4) -> &mut Self {
        self.mult(rotation)
    }

    pub fn rotate_quat(&mut self, rotation: Quat) -> &mut Self {
        self.mult(&Mat4::from_quat(rotation))
    }

    pub fn scale(&mut self, scale: Vec3) -> &mut Self {
        self.mult(&Mat4::from_scale(scale))
    }

    pub fn transform(&mut self, m: &Mat4) -> &mut Self {
        self.mult(m)
    }

    fn get_mut(&mut self) -> &mut Mat4 {
        self.stack.last_mut().expect("Stack is empty")
    }
}
