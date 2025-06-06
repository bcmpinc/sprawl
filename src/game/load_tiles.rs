use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use super::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

const MODELS: &[&str] = &[
    "models/bridge-path-a.glb",
    "models/bridge-path-b.glb",
    "models/bridge-path.glb",
    "models/building-archery.glb",
    "models/building-cabin.glb",
    "models/building-castle-path.glb",
    "models/building-farm.glb",
    "models/building-house.glb",
    "models/building-market.glb",
    "models/building-mill.glb",
    "models/building-mine.glb",
    "models/building-port.glb",
    "models/building-sheep.glb",
    "models/building-smelter.glb",
    "models/building-tower.glb",
    "models/building-village.glb",
    "models/building-watermill.glb",
    "models/building-wizard-tower.glb",
    "models/grass-forest.glb",
    "models/grass.glb",
    "models/grass-hill.glb",
    "models/grass-lumber.glb",
    "models/grass-path-corner.glb",
    "models/grass-path-intersection.glb",
    "models/grass-path-left.glb",
    "models/grass-path-right.glb",
    "models/grass-path-start.glb",
    "models/grass-path-straight.glb",
    "models/grass-rocks.glb",
    "models/river-corner.glb",
    "models/river-intersection.glb",
    "models/river-left.glb",
    "models/river-right.glb",
    "models/river-start.glb",
    "models/river-straight.glb",
    "models/stone-hill.glb",
    "models/stone-mountain.glb",
    "models/water-boat.glb",
    "models/water-corner-in.glb",
    "models/water-corner-out.glb",
    "models/water.glb",
    "models/water-island.glb",
    "models/water-river.glb",
    "models/water-rocks.glb",
    "models/water-straight.glb",
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
            let py = y as f32 * 4.0;
            commands.spawn((
                Tile::rotated(y),
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(px + 1.0, py + 1.3 , -2.0),
                RenderLayers::layer(1),
            ));
        }
    }
}
