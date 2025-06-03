#[allow(unused_braces)]
use bevy::{
    prelude::*,
};

use super::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct MousePos {
    pub hex_cell: IVec3,
    pub on_screen: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePos>();
    app.register_type::<MousePos>();
    app.add_systems(First, tracking);
}

const R: f32 = 0.57735027; // 1.0 / f32::sqrt(3.0);

const POSITION_TO_CUBE: Mat3 = Mat3::from_cols(
    vec3( 1.0,  0.0,-1.0),
    vec3( 0.0,  0.0, 0.0),
    vec3(- R ,2.0*R, -R )
);

fn round_hex(hex: Vec3) -> Vec3 {
    let mut res = Vec3::round(hex);
    let diff = Vec3::abs(hex - res);
    if diff.x > diff.y && diff.x > diff.z {
        res.x = -res.y -res.z;
    } else if diff.y > diff.z {
        res.y = -res.x -res.z;
    } else {
        res.z = -res.x -res.y;
    }
    res
}

fn tracking(
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window>,
    mut mouse_pos: ResMut<MousePos>,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {return};
    let Ok(window) = q_window.single() else {return};
    if let Some(viewport_position) = window.cursor_position() {
        let ray = camera.viewport_to_world(camera_transform, viewport_position).unwrap();
        let pos = ray.origin - ray.direction * ray.origin.y / ray.direction.y;
        let hex = POSITION_TO_CUBE * pos;
        mouse_pos.hex_cell  = round_hex(hex).as_ivec3();
        mouse_pos.on_screen = true;
    } else {
        mouse_pos.on_screen = false;
    }
}
