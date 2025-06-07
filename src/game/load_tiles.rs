use bevy::{
    render::view::RenderLayers,
    prelude::*,
};

use super::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

struct Model {
    path: &'static str,
    edges: &'static str,
}

const fn model(path: &'static str, edges: &'static str) -> Model {
    Model{path, edges}
}

const MODELS: &[Model] = &[
    model("models/bridge-path-a.glb",           "......"),
    model("models/bridge-path-b.glb",           "......"),
    model("models/building-archery.glb",        "......"),
    model("models/building-cabin.glb",          "......"),
    model("models/building-castle-path.glb",    "......"),
    model("models/building-farm.glb",           "......"),
    model("models/building-house.glb",          "......"),
    model("models/building-market.glb",         "......"),
    model("models/building-mill.glb",           "......"),
    model("models/building-mine.glb",           "......"),
    model("models/building-port.glb",           "......"),
    model("models/building-sheep.glb",          "......"),
    model("models/building-smelter.glb",        "......"),
    model("models/building-tower.glb",          "......"),
    model("models/building-village.glb",        "......"),
    model("models/building-watermill.glb",      "......"),
    model("models/building-wizard-tower.glb",   "......"),
    model("models/grass-forest.glb",            "......"),
    model("models/grass.glb",                   "......"),
    model("models/grass-hill.glb",              "......"),
    model("models/grass-lumber.glb",            "......"),
    model("models/grass-path-corner.glb",       "......"),
    model("models/grass-path-intersection.glb", "......"),
    model("models/grass-path-left.glb",         "......"),
    model("models/grass-path-right.glb",        "......"),
    model("models/grass-path-start.glb",        "......"),
    model("models/grass-path-straight.glb",     "......"),
    model("models/grass-rocks.glb",             "......"),
    model("models/river-corner.glb",            "......"),
    model("models/river-intersection.glb",      "......"),
    model("models/river-left.glb",              "......"),
    model("models/river-right.glb",             "......"),
    model("models/river-start.glb",             "......"),
    model("models/river-straight.glb",          "......"),
    model("models/stone-hill.glb",              "......"),
    model("models/stone-mountain.glb",          "......"),
    model("models/water-boat.glb",              "......"),
    model("models/water-corner-in.glb",         "......"),
    model("models/water-corner-out.glb",        "......"),
    model("models/water.glb",                   "......"),
    model("models/water-island.glb",            "......"),
    model("models/water-river.glb",             "......"),
    model("models/water-rocks.glb",             "......"),
    model("models/water-straight.glb",          "......"),
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
            GltfAssetLabel::Primitive{ mesh:0, primitive:0 }.from_asset(file.path)
        );
        for y in 0..6 {
            let py = y as f32 * 2.0;
            commands.spawn((
                Tile::rotated(y),
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(px + 1.0, py + 0.7 , -2.0),
                RenderLayers::layer(1),
            ));
        }
    }
}
