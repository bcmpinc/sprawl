use bevy::{
    prelude::*,
    render::camera::ScalingMode
};

use crate::screens::Screen;

mod map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map::plugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), setup);

    // Spawn the main camera.
    app.add_systems(Startup, (
        spawn_camera,
        spawn_light,
    ));
    app.add_systems(Update, (
        orbit_camera,
    ));
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

#[derive(Component)]
struct OrbitCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        OrbitCamera,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax { max_width: 8.0, max_height: 6.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
    ));
}

fn orbit_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<OrbitCamera>>,
) {
    let radius = 50.0;
    let elevation = 50.0;
    let speed = 0.2; // radians per second

    let angle = time.elapsed_secs() * speed;
    let x = radius * angle.cos();
    let y = elevation;
    let z = radius * angle.sin();

    let mut transform = query.single_mut().unwrap();
    *transform = Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y);
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85), // soft warm
            illuminance: 3000.0,
            ..default()
        },
        Transform::default().looking_to(-Vec3::new(1.0, 2.0, 1.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.6, 0.7, 1.0), // soft cool
            illuminance: 500.0,
            ..default()
        },
        Transform::default().looking_to(-Vec3::new(-1.0, 1.5, 1.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 1000.0,
            ..default()
        },
        Transform::default().looking_to(-Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
    ));
}
