#import bevy_pbr::view_transformations::position_ndc_to_world
#import bevy_pbr::mesh_view_bindings::view

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) position: vec2<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

/// Pass-through vertex shader, skipping camera transform.
/// Used for rendering a full screen triangle.
@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var res: VertexOutput;
    res.clip_position = vec4<f32>(in.position, 1.0);
    let a = view.world_from_clip * vec4(in.position.xy, -1.0, 1.0);
    let b = view.world_from_clip * vec4(in.position.xy,  1.0, 1.0);
    res.position = ((a*b.y - b*a.y) / (a.y - b.y)).xz;
    return res;
}

fn rgb(r:f32,g:f32,b:f32) -> vec4<f32> {
    return vec4<f32>(r,g,b,1.0);
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var res: FragmentOutput;
    let pos = in.position;
    let w = fwidth(pos);
    let g = 2.0 - abs(vec2(0.5,0.5) - fract(pos)) / w;
    let t = max(g.x, g.y);
    res.color = vec4(t,t,t, 1.0);
    //res.depth =
    return res;
}
