#import bevy_pbr::view_transformations::position_ndc_to_world
#import bevy_pbr::mesh_view_bindings::view

@group(2) @binding(0) var map_texture: texture_2d<f32>;
@group(2) @binding(1) var map_sampler: sampler;
@group(2) @binding(2) var tileset_texture: texture_2d<f32>;
@group(2) @binding(3) var tileset_sampler: sampler;
@group(2) @binding(4) var<uniform> hover: vec4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) pixel: vec4<f32>,
    @location(1) hexagon: vec3<f32>,
};

struct FragmentOutput {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

const S = sqrt(3.0) / 6.0;
const R = 1.0 / sqrt(3.0);

const POSITION_TO_CUBE: mat2x3<f32> = mat2x3<f32>(
    vec3<f32>( 1.0,  0.0,-1.0),
    vec3<f32>(- R ,2.0*R, -R ),
);

const CUBE_TO_POSITION: mat3x2<f32> = mat3x2<f32>(
    vec2<f32>( 0.5,    -S),
    vec2<f32>( 0.0, 2.0*S),
    vec2<f32>(-0.5,    -S),
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
    var out: VertexOutput;
    let a = view.world_from_clip * vec4(in.position.xy, -1.0, 1.0);
    let b = view.world_from_clip * vec4(in.position.xy,  1.0, 1.0);
    out.pixel = vec4<f32>(in.position, 1.0);
    out.hexagon = POSITION_TO_CUBE * ((a*b.y - b*a.y) / (b.y - a.y)).xz;
    return out;
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

fn sum(v: vec3<f32>) -> f32 {
    return v.x+v.y+v.z;
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    let hex = round_hex(in.hexagon);

    // Calculate a hex outline.
    let w = max3(fwidth(in.hexagon));
    let edge_distance = 1.0 - max3(SUM_OTHER * abs(in.hexagon - hex));
    let edge_color = clamp(1.0 - edge_distance / w, 0.0, 1.0);

    // Sample tile texture
    let offset = in.hexagon - hex;
    //let pixel = in.pixel.xy/100.0;
    let hex_center = vec4(CUBE_TO_POSITION * in.hexagon, 0.0, 1.0).xzyw;
    let pixel = view.clip_from_world * hex_center;

    let tile = textureSample(map_texture, map_sampler, (hex.xy + 0.5) / 32.0 + 0.5);
    //let pixel = view.clip_from_world * (vec4(CUBE_TO_POSITION * hex, 0.0, 1.0).xzyw); //in.clip_position - view.clip_from_world * vec4(CUBE_TO_POSITION * hex, 0.0, 1.0);
    var color = vec4(pixel.xy, 0.2, 1.0); //textureSample(tileset_texture, tileset_sampler, offset.xy);

    if all(abs(vec4(hex,0.0) - hover) < vec4(0.1)) {
        color = rgb(1.0,0.0,1.0);
    }

    var out: FragmentOutput;
    out.color = vec4(vec3(edge_color), 1.0);
    out.color += color;
    //out.depth =
    return out;
}
