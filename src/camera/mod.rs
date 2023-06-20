use std::ops::{Add, Mul};
use cgmath::{Point3, Vector3, Matrix4, Rad, InnerSpace};
use winit::event::*;

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
    pub frame_idx: u32,
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
            self.position.x, self.position.y, self.position.z, 0.0,
        );
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    raygen: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    frame_idx: u32,
    _padding: [u32; 3],
}

impl CameraUniform
{
    pub fn new() -> Self
    {
        use cgmath::SquareMatrix;
        Self {
            raygen: Matrix4::identity().into(),
            view_proj: Matrix4::identity().into(),
            frame_idx: 0,
            _padding: [0; 3],
        }
    }
    pub fn update(&mut self, camera: &Camera)
    {
        self.raygen = camera.raygen_matrix().into();
        self.view_proj = camera.view_projection_matrix().into();
        self.frame_idx = camera.frame_idx;
    }
}

pub struct CameraController
{
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController
{
    pub fn new(speed: f32) -> Self
    {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn update_camera(&self, camera: &mut Camera) {
        if self.is_forward_pressed {
            camera.position += camera.direction * self.speed;
        }
        if self.is_backward_pressed {
            camera.position -= camera.direction * self.speed;
        }
        let right = camera.direction.cross(camera.up).normalize();
        if self.is_left_pressed {
            camera.position -= right * self.speed;
        }
        if self.is_right_pressed {
            camera.position += right * self.speed;
        }
        if self.is_forward_pressed | self.is_backward_pressed | self.is_left_pressed | self.is_right_pressed {
            camera.frame_idx = 0;
        }
        else {
            camera.frame_idx += 1;
        }
    }
}