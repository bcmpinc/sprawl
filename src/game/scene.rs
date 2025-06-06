use bevy::{
    input::mouse::MouseMotion, prelude::*, render::{
        camera::ScalingMode,
        view::RenderLayers,
    }
};

use super::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Spawn the main camera.
    app.add_systems(Startup, (
        spawn_camera,
        spawn_light,
    ));
    app.add_systems(PreUpdate, (
        orbit_camera,
    ));
}

#[derive(Component)]
pub struct MainCamera {
    pub next_transform: Transform,
    yaw: f32,   // horizontal angle in radians
    pitch: f32, // vertical angle in radians
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        MainCamera{
            next_transform: default(),
            yaw: 0.0,
            pitch: -1.0,
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 2.0/TILE_SIZE as f32,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
    ));
}

fn orbit_camera(
    mut query: Query<(&mut Transform, &mut MainCamera)>,
    mut motion: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let Ok((mut transform, mut main)) = query.single_mut() else { return };

    let mut delta = Vec2::ZERO;
    for ev in motion.read() {
        delta += ev.delta;
    }

    // Only update if right mouse button is pressed
    if mouse_button.pressed(MouseButton::Middle) {
        // Sensitivity can be tuned
        let sensitivity = 0.005;
        main.yaw -= delta.x * sensitivity;
        main.pitch = (main.pitch - delta.y * sensitivity).clamp(-1.5, -0.5); // clamp pitch to avoid gimbal lock
    }

    if mouse_button.pressed(MouseButton::Right) {

    }

    let rotation = Quat::from_euler(EulerRot::YXZ, main.yaw, main.pitch, 0.0);

    *transform = main.next_transform;
    main.next_transform.rotation = rotation;
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
