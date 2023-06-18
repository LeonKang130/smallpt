use std::ops::{Add, Mul};
use cgmath::{Point3, Vector3, Matrix4, Rad};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> Matrix4<f32>
    {
        return Matrix4::look_at_rh(self.position, self.position.add(self.direction), self.up);
    }
    pub fn projection_matrix(&self) -> Matrix4<f32>
    {
        let aspect = self.width / self.height;
        let fovy = Rad(2.0 * (0.5 * self.height / self.focal_length).atan());
        return cgmath::perspective(fovy, aspect, self.znear, self.zfar);
    }
    pub fn view_projection_matrix(&self) -> Matrix4<f32>
    {
        return OPENGL_TO_WGPU_MATRIX * self.view_matrix() * self.projection_matrix();
    }
    pub fn raygen_matrix(&self) -> Matrix4<f32>
    {
        let up = self.up.mul(0.5 * self.height);
        let forward = self.direction.mul(self.focal_length);
        let right = self.direction.cross(self.up).mul(0.5 * self.width);
        return Matrix4::new(
            right.x, right.y, right.z, 0.0,
            up.x, up.y, up.z, 0.0,
            forward.x, forward.y, forward.z, 0.0,
            self.position.x, self.position.y, self.position.z, 0.0
        );
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    raygen: [[f32; 4]; 4],
}

impl CameraUniform
{
    pub fn new() -> Self
    {
        use cgmath::SquareMatrix;
        Self {
            raygen: Matrix4::identity().into(),
        }
    }
    pub fn update_raygen_matrix(&mut self, camera: &Camera)
    {
        self.raygen = camera.raygen_matrix().into();
    }
}
