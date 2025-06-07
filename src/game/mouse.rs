use bevy::{
    picking::{
        backend::{ray::RayMap, HitData, PointerHits},
        PickSet
    },
    prelude::*,
    render::extract_resource::{ExtractResource, ExtractResourcePlugin},
};

use super::prelude::*;

#[derive(Resource, Default, Reflect, ExtractResource, Clone)]
#[reflect(Resource)]
pub struct MousePos {
    pub hex_cell: IVec3,
    pub on_screen: bool,
    pub click: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePos>();
    app.register_type::<MousePos>();
    app.add_plugins(ExtractResourcePlugin::<MousePos>::default());
    app.add_systems(PreUpdate, tracking.in_set(PickSet::Backend));
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

/// Casts rays into the scene using [`MeshPickingSettings`] and sends [`PointerHits`] events.
pub fn tracking(
    ray_map: Res<RayMap>,
    main_camera: Query<&Camera, With<MainCamera>>,
    map: Query<Entity, With<TileMap>>,
    mut output: EventWriter<PointerHits>,
    mut mouse_pos: ResMut<MousePos>,
) {
    mouse_pos.on_screen = false;
    let Ok(entity) = map.single() else {return};
    for (&ray_id, &ray) in ray_map.iter() {
        let Ok(camera) = main_camera.get(ray_id.camera) else {
            continue;
        };
        let depth = ray.origin.y / ray.direction.y;
        let position = ray.origin - ray.direction * depth;
        let hit = (
            entity,
            HitData{
                camera: ray_id.camera,
                depth,
                position: Some(position),
                normal: Some(Vec3::Y)
            },
        );
        let picks = vec![hit];

        output.write(PointerHits{
            pointer: ray_id.pointer,
            picks,
            order: camera.order as f32 - 0.1,
        });

        // Update hovered hexagon
        let hex = POSITION_TO_CUBE * position;
        mouse_pos.hex_cell  = round_hex(hex).as_ivec3();
        mouse_pos.on_screen = true;
    }
}
