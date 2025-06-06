use bevy::prelude::*;

mod load_tiles;
mod map;
mod mouse;
mod scene;
mod tileset;

#[allow(unused_imports)]
mod prelude {
    pub use super::scene::MainCamera;
    pub use super::mouse::MousePos;
    pub use super::tileset::{Tileset, Tile};

    pub const TILE_SIZE: u32 = 64;
    pub const TILE_COUNT: u32 = 48;
    pub const MAP_SIZE: u32 = 1024;
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map::MapPlugin,
        mouse::plugin,
        scene::plugin,
        tileset::plugin,
        load_tiles::plugin,
    ));
}
