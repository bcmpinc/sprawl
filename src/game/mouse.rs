use bevy::{
    picking::{
        backend::{ray::RayMap, HitData, PointerHits},
        PickSet
    },
    prelude::*,
    render::extract_resource::{ExtractResource, ExtractResourcePlugin},
    window::PrimaryWindow,
};

use super::prelude::*;

#[derive(Resource, Default, Reflect, ExtractResource, Clone)]
#[reflect(Resource)]
pub struct MousePos {
    pub hex_cell: IVec3,
    pub on_screen: bool,
    pub click_started: Option<Vec2>,
    pub click: bool,
    pub selected_tile: UVec2,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MousePos>();
    app.register_type::<MousePos>();
    app.add_plugins(ExtractResourcePlugin::<MousePos>::default());
    app.add_systems(PreUpdate, tracking.in_set(PickSet::Backend));
    app.add_systems(First, |mut mousepos: ResMut<MousePos>| {mousepos.click = false;});
}

/// Casts rays into the scene using [`MeshPickingSettings`] and sends [`PointerHits`] events.
pub fn tracking(
    ray_map: Res<RayMap>,
    main_camera: Query<&Camera, With<MainCamera>>,
    map: Query<Entity, With<TileMap>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut output: EventWriter<PointerHits>,
) {
    let Ok(entity) = map.single() else {return};

    // Because Raymap contains rays for mouse cursors that are no longer inside the window.
    if window.single().unwrap().cursor_position().is_none() {return}

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
    }
}
