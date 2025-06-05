@group(0) @binding(0) var cells: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    var state = textureLoad(cells, vec2<i32>(gid.xy));
    state.r += f32(gid.x) / 256.0;
    state.g += f32(gid.y) / 256.0;
    state.b -= f32(gid.x + gid.y) / 256.0;
    state = fract(state);
    //textureStore(cells, vec2<i32>(gid.xy), vec4(state.r, state.g, state.b, 1.0));
}
