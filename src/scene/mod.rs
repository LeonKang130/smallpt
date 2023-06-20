#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere
{
    radius: f32,
    material_idx: u32,
    _padding1: [u32; 2],
    center: [f32; 3],
    _padding2: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material
{
    color: [f32; 3],
    _padding1: u32,
    emission: [f32; 3],
    clean_coat: f32,
}

impl Sphere
{
    pub const fn new(radius: f32, center: [f32; 3], material_idx: u32) -> Self
    {
        Self {
            radius,
            material_idx,
            _padding1: [0; 2],
            center,
            _padding2: 0,
        }
    }
}

impl Material
{
    pub const fn new(color: [f32; 3], emission: [f32; 3], clean_coat: f32) -> Self
    {
        Self {
            color,
            _padding1: 0,
            emission,
            clean_coat,
        }
    }
}

pub const SPHERES: &[Sphere] = &[
    Sphere::new(10000.0, [275.0, -10000.0, 275.0], 0), // floor
    Sphere::new(10000.0, [275.0, 10550.0, 275.0], 0), // ceiling
    Sphere::new(10000.0, [275.0, 275.0, 10550.0], 0), // back wall
    Sphere::new(10000.0, [10550.0, 275.0, 275.0], 1), // left
    Sphere::new(10000.0, [-10000.0, 275.0, 275.0], 2), // right
    Sphere::new(5000.0, [275.0, 5549.5, 275.0], 3), // light
    Sphere::new(100.0, [275.0, 100.0, 275.0], 4), // ball
];
pub const MATERIALS: &[Material] = &[
    Material::new([0.725, 0.71, 0.68], [0.0; 3], 0.0), // white
    Material::new([0.63, 0.065, 0.05], [0.0; 3], 0.0), // red
    Material::new([0.08, 0.12, 0.75], [0.0; 3], 0.0), // blue
    Material::new([0.0; 3], [10.0; 3], 0.0), // light
    Material::new([0.88, 0.55, 0.08], [0.0; 3], 0.2), // glossy yellow
];