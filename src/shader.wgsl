struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) frag_coord: vec2<f32>,
};
@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(vertex_index)) * 0.5;
    let y = f32(i32(vertex_index & 1u) * 2 - 1) * 0.5;
    let clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.clip_position = clip_position;
    out.frag_coord = clip_position.rg;
    return out;
}
@fragment
fn fragment_main(@location(0) frag_coord: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(frag_coord * 0.49 + 0.5, 1.0, 1.0);
}