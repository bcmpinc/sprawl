use bevy::{
    asset::RenderAssetUsages,
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_resource::{AsBindGroup, ShaderRef}}
};

use crate::screens::Screen;

use super::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TilemapMaterial>();
    app.add_plugins(MaterialPlugin::<TilemapMaterial>{
        prepass_enabled: false,
        shadows_enabled: false,
        ..default()
    });
    app.add_systems(OnEnter(Screen::Gameplay), setup);
    app.add_systems(Update, update_tile);
}

/**
 * Shader for drawing the tilemap.
 */
#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct TilemapMaterial {
    #[texture(0)] #[sampler(1)] tiles: Handle<Image>,
    #[uniform(2)] hover_tile: Vec3,
}

impl Material for TilemapMaterial {
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
    assets: Res<AssetServer>,
) {
    // Fullscreen triangle (covers full screen)
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        // triangle that covers full screen in clip space
        [-1.0, -3.0, 0.0],
        [ 3.0,  1.0, 0.0],
        [-1.0,  1.0, 0.0],
    ]);

    let tiles : Handle<Image> = assets.load_with_settings(
        "images/ducky_shear.png",
        |settings: &mut ImageLoaderSettings| {
            // Use `nearest` image sampling to preserve pixel art style.
            settings.sampler = ImageSampler::nearest();
        },
    );

    commands.spawn((
        Name::new("Tilemap"),
        Mesh3d(meshes.add(mesh).into()),
        MeshMaterial3d(materials.add(TilemapMaterial{
            tiles,
            hover_tile: Vec3::ZERO,
        })),
        Transform::IDENTITY,
    ));
}

fn update_tile(mouse: Res<MousePos>, mut materials: ResMut<Assets<TilemapMaterial>>) {
    let tile = mouse.hex_cell.as_vec3();
    for mat in materials.iter_mut() {
        mat.1.hover_tile = tile;
    }
}
