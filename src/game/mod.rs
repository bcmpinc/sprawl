#[allow(unused_braces)]
use bevy::{
    render::view::RenderLayers,
    prelude::*,
};
use tiles::Tile;

use crate::screens::Screen;

pub const TILE_SIZE: u32 = 128;
pub const TILE_COUNT: u32 = 48;

mod map;
mod mouse;
mod scene;
mod tiles;

#[allow(unused_imports)]
mod prelude {
    pub use super::scene::MainCamera;
    pub use super::mouse::MousePos;
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map::MapPlugin,
        mouse::plugin,
        scene::plugin,
        tiles::plugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

const MODELS: &[&str] = &[
    "models/building-smelter.glb",
    "models/grass-forest.glb",
    "models/grass-rocks.glb",
    "models/grass-path-straight.glb",
];

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load tile material
    let texture: Handle<Image> = asset_server.load("images/colormap.png");
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        ..default()
    });

    // Create tiles
    for (x,file) in MODELS.iter().enumerate() {
        let px = x as f32 * 2.0;
        let mesh: Handle<Mesh> = asset_server.load(
            GltfAssetLabel::Primitive{ mesh:0, primitive:0 }.from_asset(*file)
        );
        for y in 0..6 {
            let py = y as f32 * 2.0;
            commands.spawn((
                Tile::rotated(y),
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(px + 1.0, py + 0.5 , -2.0),
                RenderLayers::layer(1),
            ));
        }
    }
}
