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
var<private> seed: u32;
const PI = 3.1415926;
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
struct Sphere
{
    center: vec3<f32>,
    radius: f32,
    emission: vec3<f32>,
    color: vec3<f32>,
};
struct Hit
{
    primitive_idx: i32,
    t: f32,
};
struct Ray
{
    origin: vec3<f32>,
    direction: vec3<f32>,
};
fn generate_ray(frag_coord: vec2<f32>) -> Ray
{
    var ray = Ray();
    return ray;
}
fn intersect(ray: Ray) -> Hit
{
    var hit = Hit();
    hit.primitive_idx = 0;
    return hit;
}
@fragment
fn fragment_main(@location(0) frag_coord: vec2<f32>) -> @location(0) vec4<f32> {
    let uv = clamp(frag_coord * 0.5 + 0.5, vec2<f32>(0.0), vec2<f32>(1.0));
    var ifrag_coord = vec2<u32>(uv * vec2<f32>(1024.0, 768.0));
    seed = (ifrag_coord.r * 768u + ifrag_coord.g) * 0x000343fdu + 0x00269ec3u;
    var color = vec3<f32>(0.0);
    for (var i = 0; i < 16; i++)
    {
        color += vec3<f32>(frand(), frand(), frand());
    }
    return vec4<f32>(color / 16.0, 1.0);
}