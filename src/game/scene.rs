use std::{
    f64::consts::FRAC_PI_2, mem::swap
};

use bevy::{
    prelude::*,
    render::{
        camera::ScalingMode,
        view::RenderLayers,
    }
};
use bevy_editor_cam::prelude::*;

use super::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultEditorCamPlugins);

    app.register_type::<MainCamera>();

    // Spawn the main camera.
    app.add_systems(Startup, (
        spawn_camera,
        spawn_light,
    ));

    app.add_systems(Last, lag_transform);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        EditorCam{
            enabled_motion: EnabledMotion{
                pan: true,
                orbit: true,
                zoom: false,
            },
            orbit_constraint: OrbitConstraint::Fixed {
                up: Vec3::Y,
                pitch_limits: PitchLimits::new(0.5, FRAC_PI_2),
            },
            zoom_limits: default(),
            smoothing: default(),
            sensitivity: default(),
            momentum: default(),
            input_debounce: default(),
            perspective: default(),
            orthographic: default(),
            last_anchor_depth: default(),
            current_motion: default(),
        },
        MainCamera,
        LaggedTransform(default()),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 2.0/TILE_SIZE as f32,
            ..OrthographicProjection::default_3d()
        }),
        Transform{
            rotation: Quat::from_rotation_x(-1.0), // from_euler(EulerRot::YXZ, 0.0, -1.0, 0.0),
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        },
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        Tile::default(),
        InheritedVisibility::default(),
        children![
            (
                DirectionalLight {
                    color: Color::srgb(1.0, 0.95, 0.85), // soft warm
                    illuminance: 3000.0,
                    ..default()
                },
                Transform::default().looking_to(-Vec3::new(1.0, 2.0, 1.0), Vec3::Y),
                RenderLayers::layer(1),
            ),(
                DirectionalLight {
                    color: Color::srgb(0.6, 0.7, 1.0), // soft cool
                    illuminance: 500.0,
                    ..default()
                },
                Transform::default().looking_to(-Vec3::new(-1.0, 1.5, 1.0), Vec3::Y),
                RenderLayers::layer(1),
            ),(
                DirectionalLight {
                    color: Color::WHITE,
                    illuminance: 1000.0,
                    ..default()
                },
                Transform::default().looking_to(-Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
                RenderLayers::layer(1),
            )
        ],
    ));
}

#[derive(Component)]
pub struct LaggedTransform(GlobalTransform);

fn lag_transform(query: Query<(&mut GlobalTransform, &mut LaggedTransform)>) {
    for (mut trans, mut lag) in query {
        swap(&mut *trans, &mut lag.0);
    }
}
