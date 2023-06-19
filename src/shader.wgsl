struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) frag_coord: vec2<f32>,
};
@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(vertex_index & 1u) * 2 - 1);
    let y = f32(i32(vertex_index & 2u) - 1);
    let clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.clip_position = clip_position;
    out.frag_coord = clip_position.rg;
    return out;
}
struct CameraUniform
{
    raygen: mat4x4<f32>,
    frame_idx: u32,
};
struct Sphere
{
    radius: f32,
    clean_coat: f32,
    center: vec3<f32>,
    color: vec3<f32>,
    emission: vec3<f32>,
};
struct Ray
{
    origin: vec3<f32>,
    direction: vec3<f32>,
};
struct Hit
{
    primitive_idx: i32,
    t: f32,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@group(1) @binding(0)
var<storage, read> spheres: array<Sphere>;
@group(2) @binding(0)
var<storage, read_write> accumulate: array<vec3<f32>>;
var<private> seed: u32;
const PI = 3.1415926;
const EPS = 1e-3;
const SPP = 48;
const MAX_BOUNCE = 8;
fn frand() -> f32
{
    /*
        from Inigo Equilez's article on float and random
        https://iquilezles.org/articles/sfrand/
    */
    seed *= 16807u;
    return bitcast<f32>((seed >> 9u) | 0x3f800000u) - 1.0;
}
fn frand2() -> vec2<f32>
{
    return vec2<f32>(frand(), frand());
}
fn sample_cosine_hemisphere(normal: vec3<f32>) -> vec3<f32>
{
    let tangent = normalize(cross(select(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0, 1.0, 1.0), abs(normal.r) < 1e-3), normal));
    let binormal = cross(normal, tangent);
    let u2 = frand2();
    let cos_theta = sqrt(u2.r);
    let sin_theta = sqrt(1.0 - u2.r);
    let phi = 2.0 * PI * u2.g;
    return cos_theta * normal + sin_theta * (cos(phi) * tangent + sin(phi) * binormal);
}
fn generate_ray(frag_coord: vec2<f32>) -> Ray
{
    var ray: Ray;
    let offset = vec2<f32>(frand(), frand()) / 1024.0;
    ray.direction = normalize((camera.raygen * vec4<f32>(frag_coord + offset, 1.0, 0.0)).rgb);
    ray.origin = (camera.raygen * vec4<f32>(0.0, 0.0, 0.0, 1.0)).rgb;
    return ray;
}
fn intersect(ray: Ray) -> Hit
{
    var hit: Hit;
    hit.primitive_idx = -1;
    hit.t = 1e30;
    for (var i = 0; i < i32(arrayLength(&spheres)); i++)
    {
        let sphere = spheres[i];
        let f = ray.origin - sphere.center;
        let b = -dot(f, ray.direction);
        let l = f + b * ray.direction;
        let delta = sphere.radius * sphere.radius - dot(l, l);
        if (delta > 0.0) {
            let q = b + sign(b) * sqrt(delta);
            let c = dot(f, f) - sphere.radius * sphere.radius;
            let t0 = c / q;
            let t1 = q;
            if (t0 > EPS && t0 < hit.t) {
                hit.t = t0;
                hit.primitive_idx = i;
            }
            if (t1 > EPS && t1 < hit.t) {
                hit.t = t1;
                hit.primitive_idx = i;
            }
        }
    }
    return hit;
}
fn radiance(ray: Ray) -> vec3<f32>
{
    var ray = ray;
    var acc = vec3<f32>(0.0);
    var amp = vec3<f32>(1.0);
    for (var i = 0; i < MAX_BOUNCE; i++)
    {
        let hit = intersect(ray);
        if (hit.primitive_idx < 0)
        {
            break;
        }
        let sphere = spheres[hit.primitive_idx];
        ray.origin += hit.t * ray.direction;
        var normal = normalize(ray.origin - sphere.center);
        normal *= select(1.0, -1.0, dot(ray.direction, normal) > 0.0);
        ray.origin += normal * EPS;
        acc += sphere.emission * amp * PI;
        if (frand() < sphere.clean_coat)
        {
            ray.direction = reflect(ray.direction, normal);
        }
        else
        {
            amp *= sphere.color;
            ray.direction = sample_cosine_hemisphere(normal);
        }
    }
    return acc;
}
fn aces_tone_mapping(x: vec3<f32>) -> vec3<f32>
{
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
}
fn linear_to_srgb(x: vec3<f32>) -> vec3<f32>
{
    return clamp(select(1.055 * pow(x, vec3<f32>(1.0 / 2.4)) - 0.055, 12.92 * x, x <= 0.00031308), vec3<f32>(0.0), vec3<f32>(1.0));
}
@fragment
fn fragment_main(@location(0) frag_coord: vec2<f32>) -> @location(0) vec4<f32> {
    let uv = clamp(frag_coord * 0.5 + 0.5, vec2<f32>(0.0), vec2<f32>(1.0));
    var ifrag_coord = vec2<u32>(uv * 1024.0);
    let frag_idx = ifrag_coord.g | ifrag_coord.r << 10u;
    seed = (frag_idx | camera.frame_idx << 20u) * 0x000343fdu + 0x00269ec3u;
    var color = vec3<f32>(0.0);
    for (var i = 0; i < SPP; i++)
    {
        let ray = generate_ray(frag_coord);
        color += radiance(ray);
    }
    color *= 1.0 / f32(SPP);
    let accumulated_color = accumulate[frag_idx];
    color = (color + f32(camera.frame_idx) * accumulated_color) / f32(1u + camera.frame_idx);
    accumulate[frag_idx] = color;
    return vec4<f32>(linear_to_srgb(color), 1.0);
}