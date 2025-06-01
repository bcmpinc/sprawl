// @group(2) @binding(0) var<uniform> color: vec4<f32>;
// @group(2) @binding(1) var radius_texture: texture_2d<f32>;
// @group(2) @binding(2) var radius_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};


/// Pass-through vertex shader, skipping camera transform.
/// Used for rendering a full screen triangle.
@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var res: VertexOutput;
    res.position = vec4<f32>(in.position, 1.0);
    return res;
}

fn rgb(r:f32,g:f32,b:f32) -> vec4<f32> {
    return vec4<f32>(r,g,b,1.0);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var res = rgb(1.0,0.0,1.0);
    return res;
}
