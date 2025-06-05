#import bevy_pbr::mesh_view_bindings::view

@group(2) @binding(0) var map_texture: texture_2d<f32>;
@group(2) @binding(1) var tileset_texture: texture_2d<f32>;
@group(2) @binding(2) var<uniform> hover: vec4<f32>;
@group(2) @binding(3) var<uniform> tilesize: f32;

struct VertexInput {
    @location(0) pixel: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) pixel: vec4<f32>,
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
    let a = view.world_from_clip * vec4(in.pixel.xy, -1.0, 1.0);
    let b = view.world_from_clip * vec4(in.pixel.xy,  1.0, 1.0);
    let plane_pos = (a*b.y - b*a.y) / (b.y - a.y);

    var out: VertexOutput;
    let pos = vec4<f32>(in.pixel, 1.0);
    out.position = pos;
    out.pixel = view.view_from_clip * pos;
    // out.pixel2 = view.clip_from_world * view.world_from_clip * out.pixel; //vec4(plane_pos);
    // out.pixel2 = view.clip_from_world * view.world_from_clip * out.pixel; //vec4(plane_pos);
    // out.pixel2 = view.clip_from_world * view.world_from_clip * out.pixel; //vec4(plane_pos);
    // out.pixel2 = view.clip_from_world * view.world_from_clip * out.pixel; //vec4(plane_pos);
    out.hexagon = POSITION_TO_CUBE * plane_pos.xz;
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

const OFFSETS: array<vec3<f32>, 7> = array<vec3<f32>, 7>(
    vec3<f32>( 0, 0, 0),
    vec3<f32>(-1, 1, 0),
    vec3<f32>( 1,-1, 0),
    vec3<f32>(-1, 0, 1),
    vec3<f32>( 1, 0,-1),
    vec3<f32>( 0,-1, 1),
    vec3<f32>( 0, 1,-1),
);

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    let center_hex = round_hex(in.hexagon);

    // Calculate a hex outline.
    let w = max3(fwidth(in.hexagon));
    let edge_distance = 1.0 - max3(SUM_OTHER * abs(in.hexagon - center_hex));
    let edge_color = clamp(1.0 - edge_distance / w, 0.0, 1.0);

    // Sample tile texture
    var color = vec4(0.5);
    var depth = -10.0;
    var tile_offset = vec2<i32>(i32(tilesize*2.0), 0);
    for (var i = 0; i < 7; i += 1) {
        let hex = center_hex + OFFSETS[i];
        let hex_position = vec4(CUBE_TO_POSITION * hex, 0.0, 1.0).xzyw;
        let position = in.pixel - view.view_from_world * hex_position;
        if position.x < -1.0 || 1.0 < position.x {
            continue;
        }
        let offset = position.xy * vec2(tilesize,-tilesize) + tilesize * vec2(1.0,1.5) + vec2(0.5, 0.5);

        let tile = textureLoad(map_texture, vec2<i32>(hex.xy+16.5) % 32, 0);
        let tile_id = i32(dot(tile, vec4(1234.,432.,6234.,123.))) % 4;
        let new_color = textureLoad(tileset_texture, vec2<i32>(offset) + tile_id*tile_offset, 0);
        if new_color.a > 0.5 && depth < position.y {
            color = new_color;
            depth = position.y;
        }
    }

    if all(abs(vec4(center_hex,0.0) - hover) < vec4(0.1)) {
        color = rgb(1.0,0.0,1.0);
    }

    var out: FragmentOutput;
    out.color = vec4(vec3(edge_color), 1.0);
    out.color = color;
    //out.depth =
    return out;
}
