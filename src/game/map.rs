use std::borrow::Cow;

use bevy::{
    asset::{AssetLoader, RenderAssetUsages},
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_resource::{AsBindGroup, BindGroupLayout, CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderRef},
        renderer::RenderDevice,
        RenderApp,
    },
};

use crate::screens::Screen;

use super::prelude::*;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TilemapMaterial>();
        app.add_plugins(MaterialPlugin::<TilemapMaterial>{
            prepass_enabled: false,
            shadows_enabled: false,
            ..default()
        });
        app.add_systems(OnEnter(Screen::Gameplay), setup);
        app.add_systems(Update, update_tile);
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<KernelPipeline>();
            println!("KernelPipeline added");
        }
    }
}

/**
 * Shader for drawing the tilemap.
 */
#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct TilemapMaterial {
    #[texture(0)] #[sampler(1)] tiles: Handle<Image>,
    #[uniform(2)] hover_tile: Vec4,
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
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(TilemapMaterial{
            tiles: tiles.clone(),
            hover_tile: Vec4::ZERO,
        })),
        Transform::IDENTITY,
    ));

    let data = ShaderData {
        tiles
    };
}

fn update_tile(mouse: Res<MousePos>, mut materials: ResMut<Assets<TilemapMaterial>>) {
    let tile = mouse.hex_cell.as_vec3();
    for mat in materials.iter_mut() {
        mat.1.hover_tile = tile.extend(
            if mouse.on_screen {0.0} else {1.0}
        ) ;
    }
}



#[derive(TypePath,AsBindGroup)]
struct ShaderData {
    #[storage_texture(0)] tiles: Handle<Image>,
}

#[derive(Resource)]
struct KernelPipeline {
    pub pipeline: CachedComputePipelineId,
    pub bind_group_layout: BindGroupLayout,
}

impl FromWorld for KernelPipeline {
    fn from_world(world: &mut World) -> Self {
        // Gather resources
        let assets = world.get_resource::<AssetServer>().unwrap();
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let cache = world.get_resource::<PipelineCache>().unwrap();

        // Build shader graph
        let shader: Handle<Shader> = assets.load("shaders/simulate.wgsl");
        let bind_group_layout = ShaderData::bind_group_layout(&render_device);
        let pipeline = cache.queue_compute_pipeline(ComputePipelineDescriptor{
            label: None,
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: vec![],
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("main"),
            zero_initialize_workgroup_memory: true,
        });

        KernelPipeline{
            bind_group_layout,
            pipeline,
        }
    }
}
