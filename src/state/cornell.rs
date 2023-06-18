#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere
{
    radius: f32,
    _padding1: [u32; 3],
    center: [f32; 3],
    _padding2: u32,
    color: [f32; 3],
    _padding3: u32,
    emission: [f32; 3],
    _padding4: u32,
}
impl Sphere
{
    pub const fn new(radius: f32, center: [f32; 3], color: [f32; 3], emission: [f32; 3]) -> Self
    {
        Self {
            radius,
            _padding1: [0; 3],
            center,
            _padding2: 0,
            color,
            _padding3: 0,
            emission,
            _padding4: 0,
        }
    }
}
pub const SPHERES: &[Sphere] = &[
    Sphere::new(10000.0, [275.0, -10000.0, 275.0], [0.725, 0.71, 0.68], [0.0; 3]), // floor
    Sphere::new(10000.0, [275.0, 10550.0, 275.0], [0.725, 0.71, 0.68], [0.0; 3]), // ceiling
    Sphere::new(10000.0, [275.0, 275.0, 10550.0], [0.725, 0.71, 0.68], [0.0; 3]), // back wall
    Sphere::new(10000.0, [10550.0, 275.0, 275.0], [0.63, 0.065, 0.05], [0.0; 3]), // left
    Sphere::new(10000.0, [-10000.0, 275.0, 275.0], [0.08, 0.12, 0.75], [0.0; 3]), // right
    Sphere::new(5000.0, [275.0, 5549.5, 275.0], [0.0; 3], [10.0; 3]), // light
    Sphere::new(100.0, [275.0, 100.0, 275.0], [0.725, 0.71, 0.68], [0.0; 3]), // ball
];