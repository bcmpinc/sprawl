#import bevy_pbr::view_transformations::position_ndc_to_world
#import bevy_pbr::mesh_view_bindings::view

@group(2) @binding(0) var tiles_texture: texture_2d<f32>;
@group(2) @binding(1) var tiles_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) position: vec3<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

const R = 1.0 / sqrt(3.0);

const POSITION_TO_CUBE: mat2x3<f32> = mat2x3<f32>(
    vec3<f32>( 1.0,  0.0,-1.0),
    vec3<f32>(- R ,2.0*R, -R )
);

const SUM_OTHER: mat3x3<f32> = mat3x3<f32>(
    vec3<f32>(0.0,1.0,1.0),
    vec3<f32>(1.0,0.0,1.0),
    vec3<f32>(1.0,1.0,0.0),
);

/// Pass-through vertex shader, skipping camera transform.
/// Used for rendering a full screen triangle.
@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var res: VertexOutput;
    res.clip_position = vec4<f32>(in.position, 1.0);
    let a = view.world_from_clip * vec4(in.position.xy, -1.0, 1.0);
    let b = view.world_from_clip * vec4(in.position.xy,  1.0, 1.0);
    res.position = POSITION_TO_CUBE * ((a*b.y - b*a.y) / (a.y - b.y)).xz;
    return res;
}

fn rgb(r:f32,g:f32,b:f32) -> vec4<f32> {
    return vec4<f32>(r,g,b,1.0);
}

fn min3(p: vec3<f32>) -> f32 {
    return min(min(p.x, p.y), p.z);
}

fn max3(p: vec3<f32>) -> f32 {
    return max(max(p.x, p.y), p.z);
}

fn round_hex(hex: vec3<f32>) -> vec3<f32> {
    var res = round(hex);
    let diff = abs(hex - res);
    if diff.x > diff.y && diff.x > diff.z {
        res.x = -res.y -res.z;
    } else if diff.y > diff.z {
        res.y = -res.x -res.z;
    } else {
        res.z = -res.x -res.y;
    }
    return res;
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var res: FragmentOutput;
    var pos = in.position;
    let w = max3(fwidth(pos));
    let hex = round_hex(pos);
    let t = max3(SUM_OTHER * abs(pos - hex));
    let s = clamp(1.0 - (1.0 - t) / w, 0.0, 1.0);
    res.color = vec4(s,s,s, 1.0);
    let color = textureSample(tiles_texture, tiles_sampler, (hex.xy + 0.5) / 32.0 + 0.5);
    res.color += color;
    //res.depth =
    return res;
}
