#[allow(unused_braces)]
use bevy::{
    prelude::*,
};

use crate::screens::Screen;

mod map;
mod mouse;
mod scene;

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
    ));

    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/building-smelter.glb"))),
    ));
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/building-smelter.glb"))),
        Transform::from_xyz(1.0, 0.0, 0.0),
    ));
}
