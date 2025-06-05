use bevy::{render::view::RenderLayers, scene::SceneInstanceReady};
#[allow(unused_braces)]
use bevy::{
    prelude::*,
};

use crate::screens::Screen;

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

    app.add_observer(insert_render_layer);

    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

const MODELS: &[&'static str] = &[
    "models/building-smelter.glb",
    "models/grass-forest.glb",
    "models/stone-rocks.glb",
];

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (i,file) in MODELS.iter().enumerate() {
        commands.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(*file))),
            Transform::from_xyz(1.0 + 2.0 * (i as f32), 0.5, -2.0),
            RenderLayers::layer(1),
        ));
    }
}

fn insert_render_layer(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    mesh: Query<(), With<Mesh3d>>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    for child in children.iter_descendants(entity) {
        if mesh.contains(child) {
            commands.entity(child).insert(RenderLayers::layer(1));
        }
    }
}
