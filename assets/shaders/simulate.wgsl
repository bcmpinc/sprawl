@group(0) @binding(0) var cells: texture_storage_2d<rgba8uint, read_write>;

@compute @workgroup_size(8,8)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    var state = textureLoad(cells, vec2<i32>(gid.xy));
    state.r += gid.x;
    state.g += gid.y;
    state.b -= gid.x + gid.y;
    state = state % 256;
    //textureStore(cells, vec2<i32>(gid.xy), vec4(state.r, state.g, state.b, 1.0));
}
