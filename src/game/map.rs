use bevy::{
    asset::RenderAssetUsages, prelude::*, render::{mesh::PrimitiveTopology, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<TilemapMaterial>::default());
    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

/**
 * Shader for drawing the tilemap.
 */
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TilemapMaterial {
    //#[uniform(0)] color: LinearRgba,
    //#[texture(1)] #[sampler(2)] radius: Handle<Image>,
}


impl Material2d for TilemapMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
) {
    // Fullscreen triangle (covers full screen)
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        // triangle that covers full screen in clip space
        [-1.0, -3.0, 0.0],
        [3.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
    ]);

    commands.spawn((
        Name::new("Tilemap"),
        Mesh2d(meshes.add(mesh).into()),
        MeshMaterial2d(materials.add(TilemapMaterial{})),
        Transform::IDENTITY,
    ));
}
