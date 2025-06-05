use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        view::RenderLayers,
    }
};

pub(super) fn plugin(app: &mut App) {
    // Spawn the main camera.
    app.add_systems(Startup, (
        spawn_camera,
        spawn_light,
    ));
    app.add_systems(Update, (
        orbit_camera,
    ));
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        MainCamera,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0/32.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
    ));
}

fn orbit_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = query.single_mut() else {return};

    let radius = 50.0;
    let elevation = 50.0;
    let speed = 0.2; // radians per second

    let angle = time.elapsed_secs() * speed;
    let x = radius * angle.cos();
    let y = elevation;
    let z = radius * angle.sin();

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
        RenderLayers::from_layers(&[0,1]),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.6, 0.7, 1.0), // soft cool
            illuminance: 500.0,
            ..default()
        },
        Transform::default().looking_to(-Vec3::new(-1.0, 1.5, 1.0), Vec3::Y),
        RenderLayers::from_layers(&[0,1]),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 1000.0,
            ..default()
        },
        Transform::default().looking_to(-Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
        RenderLayers::from_layers(&[0,1]),
    ));
}
