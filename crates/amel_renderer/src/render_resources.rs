use amel_gpu::prelude::*;
use amel_math::prelude::*;
use amel_mesh::prelude::*;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use std::sync::Mutex;

#[derive(Debug)]
pub struct RenderResource {
    circle: Arc<GpuMesh>,
    wire_circle: Arc<GpuMesh>,

    rectangle: Arc<GpuMesh>,
    wire_rectangle: Arc<GpuMesh>,

    sphere: Arc<GpuMesh>,
    wire_sphere: Arc<GpuMesh>,

    cuboid: Arc<GpuMesh>,
    wire_cuboid: Arc<GpuMesh>,
    // cylinder: Arc<GpuMesh>,
    // cone: Arc<GpuMesh>,
    // torus: Arc<GpuMesh>,
}

impl RenderResource {
    pub fn new(device: &wgpu::Device) -> Self {
        let circle = Arc::new(Circle::new(1.0).to_mesh_builder().build().to_gpu(device));
        let wire_circle = Arc::new(
            Circle::new(1.0)
                .to_mesh_builder()
                .wireframe(true)
                .build()
                .to_gpu(device),
        );

        let rectangle = Arc::new(
            Rectangle::new(1.0, 1.0)
                .to_mesh_builder()
                .build()
                .to_gpu(device),
        );
        let wire_rectangle = Arc::new(
            Rectangle::new(1.0, 1.0)
                .to_mesh_builder()
                .wireframe(true)
                .build()
                .to_gpu(device),
        );

        let sphere = Arc::new(Sphere::new(1.0).to_mesh_builder().build().to_gpu(device));
        let wire_sphere = Arc::new(
            Sphere::new(1.0)
                .to_mesh_builder()
                .wireframe(true)
                .build()
                .to_gpu(device),
        );

        let cuboid = Arc::new(
            Cuboid::new(1.0, 1.0, 1.0)
                .to_mesh_builder()
                .build()
                .to_gpu(device),
        );
        let wire_cuboid = Arc::new(
            Cuboid::new(1.0, 1.0, 1.0)
                .to_mesh_builder()
                .wireframe(true)
                .build()
                .to_gpu(device),
        );

        Self {
            circle,
            wire_circle,
            rectangle,
            wire_rectangle,
            sphere,
            wire_sphere,
            cuboid,
            wire_cuboid,
        }
    }

    pub fn circle(&self) -> Arc<GpuMesh> {
        self.circle.clone()
    }

    pub fn wire_circle(&self) -> Arc<GpuMesh> {
        self.wire_circle.clone()
    }

    pub fn rectangle(&self) -> Arc<GpuMesh> {
        self.rectangle.clone()
    }

    pub fn wire_rectangle(&self) -> Arc<GpuMesh> {
        self.wire_rectangle.clone()
    }
}

pub static RENDER_RESOURCES: OnceCell<Mutex<RenderResource>> = OnceCell::new();

pub fn initialize_render_resources(device: &wgpu::Device) {
    RENDER_RESOURCES
        .set(Mutex::new(RenderResource::new(device)))
        .unwrap();
}
