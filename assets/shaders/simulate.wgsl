@group(0) @binding(0) var cells: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let state = textureLoad(cells, vec2<i32>(gid.xy));
    textureStore(cells, vec2<i32>(gid.xy), vec4(1.0 - state.r, state.g, state.b, 1.0));
}
