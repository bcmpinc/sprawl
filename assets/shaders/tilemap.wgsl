#import bevy_pbr::view_transformations::{
    direction_clip_to_world,
    position_clip_to_view,
    position_clip_to_world,
    position_world_to_clip,
    position_world_to_view,
}

@group(2) @binding(0) var map_texture: texture_storage_2d<rgba8uint, read>;
@group(2) @binding(1) var tileset_texture: texture_2d<f32>;
@group(2) @binding(2) var tileset_sampler: sampler;
@group(2) @binding(3) var<uniform> hover: vec4<f32>;
@group(2) @binding(4) var<uniform> tilesize: f32;
@group(2) @binding(5) var<uniform> tilecount: f32;
@group(2) @binding(6) var<uniform> selected: vec2<u32>;

struct VertexInput {
    @location(0) clip_pos: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) view_pos: vec3<f32>,
    @location(2) hexagon: vec3<f32>,
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
    let clip_pos  = vec4(in.clip_pos.xy, 0.0, 1.0);
    let origin    = position_clip_to_world(clip_pos);
    let direction = direction_clip_to_world(vec4(0.0, 0.0, 1.0, 0.0));
    let depth     = origin.y / direction.y;
    let position  = origin - direction * depth;

    var out: VertexOutput;
    out.position = vec4<f32>(in.clip_pos, 1.0);
    out.view_pos = position_clip_to_view(clip_pos);
    out.hexagon = POSITION_TO_CUBE * position.xz;
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

const OFFSETS: array<vec3<f32>, 19> = array<vec3<f32>, 19>(
    vec3<f32>( 0, 0, 0),
    vec3<f32>(-1, 1, 0),
    vec3<f32>( 1,-1, 0),
    vec3<f32>(-1, 0, 1),
    vec3<f32>( 1, 0,-1),
    vec3<f32>( 0,-1, 1),
    vec3<f32>( 0, 1,-1),
    vec3<f32>( 2,-1,-1),
    vec3<f32>(-1, 2,-1),
    vec3<f32>(-1,-1, 2),
    vec3<f32>(-2, 1, 1),
    vec3<f32>( 1,-2, 1),
    vec3<f32>( 1, 1,-2),
    vec3<f32>(-2, 2, 0),
    vec3<f32>( 2,-2, 0),
    vec3<f32>(-2, 0, 2),
    vec3<f32>( 2, 0,-2),
    vec3<f32>( 0,-2, 2),
    vec3<f32>( 0, 2,-2),
);

fn multiply_alpha(c:vec4<f32>) -> vec4<f32> {
    return vec4(c.rgb * c.a, c.a);
}

// Alphablend for colors with pre-multiplied alpha
// Color a is placed on top
fn blend(a:vec4<f32>, b:vec4<f32>) -> vec4<f32> {
    return a + b * (1.0 - a.a);
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    let center_hex = round_hex(in.hexagon);

    // Calculate a hex outline.
    // let w = max3(fwidth(in.hexagon));
    // let edge_distance = 1.0 - max3(SUM_OTHER * abs(in.hexagon - center_hex));
    // let edge_color = clamp(edge_distance / w / 4.0 - 0.1, 0.0, 0.5);

    // Sample tile texture
    var color = vec4(0.0); // vec4(vec3(edge_color), 1.0);
    var depth = -10.0;
    var tile_scale = vec2(1.0 / tilecount, 1.0/6.0);

    for (var i = 0; i < 19; i += 1) {
        let hex = center_hex + OFFSETS[i];
        let hex_position = vec3(CUBE_TO_POSITION * hex, 0.0).xzy;
        let position = in.view_pos - position_world_to_view(hex_position);
        if position.x < -0.6 || 0.6 < position.x || position.y < -0.6 || 0.85 < position.y {
            continue;
        }
        let offset = (0.5 * position.xy * vec2(1.0,-1.0) + vec2(0.5,0.65));

        var tile = selected;
        let is_hover = all(abs(vec4(hex,0.0) - hover) < vec4(0.1));
        if !is_hover {
            tile = textureLoad(map_texture, vec2<i32>(hex.xy) & vec2(1023)).rg;
        }
        let tile_id  = f32(tile.r);
        let tile_rot = f32(tile.g);

        var new_color = textureSample(tileset_texture, tileset_sampler, (offset + vec2(tile_id, tile_rot))*tile_scale);
        if new_color.a > 0.1 && depth < position.y {
            if is_hover {
                new_color = blend(0.2 * rgb(1.0,0.0,1.0), new_color);
            }
            color = blend(new_color, color);
            depth = position.y;
        }
    }

    var out: FragmentOutput;
    out.color = color / color.a;
    return out;
}
