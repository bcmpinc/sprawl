use std::borrow::Cow;

use bevy::{
    asset::{LoadState, RenderAssetUsages},
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        mesh::PrimitiveTopology,
        render_asset::RenderAssets,
        render_graph::Node,
        render_resource::{AsBindGroup, BindGroup, BindGroupLayout, CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderRef, TextureUsages},
        renderer::RenderDevice,
        storage::GpuShaderStorageBuffer,
        texture::{FallbackImage, GpuImage},
        Render, RenderApp, RenderSet
    }
};

use crate::screens::Screen;

use super::prelude::*;

pub(super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TilemapMaterial>();
        app.add_plugins(ExtractResourcePlugin::<ShaderData>::default());
        app.add_plugins(MaterialPlugin::<TilemapMaterial>{
            prepass_enabled: false,
            shadows_enabled: false,
            ..default()
        });
        app.add_systems(OnEnter(Screen::Gameplay), setup);
        app.add_systems(Update, update_tile);
        app.add_systems(Update, configure_image);
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<KernelPipeline>();
            render_app.add_systems(Render, prepare_compute.in_set(RenderSet::Prepare));
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
            settings.is_srgb = false;
        },
    );

    commands.spawn((
        Name::new("Tilemap"),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(TilemapMaterial{
            tiles,
            hover_tile: Vec4::ZERO,
        })),
        Transform::IDENTITY,
    ));
}

fn update_tile(mouse: Res<MousePos>, mut materials: ResMut<Assets<TilemapMaterial>>) {
    let tile = mouse.hex_cell.as_vec3();
    for mat in materials.iter_mut() {
        mat.1.hover_tile = tile.extend(
            if mouse.on_screen {0.0} else {1.0}
        ) ;
    }
}

#[derive(TypePath,AsBindGroup,Resource,Clone,ExtractResource)]
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

pub struct DispatchKernel;

#[derive(Debug, Resource)]
pub struct KernelBindGroup(pub BindGroup);



fn configure_image(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    material: Res<Assets<TilemapMaterial>>,
    asset_server: Res<AssetServer>,
    shader_data: Option<Res<ShaderData>>,
) {
    if shader_data.is_some() {return}
    let Some(material) = &material.iter().next() else {return};
    let handle = material.1.tiles.id();

    // Wait until asset is fully loaded
    if matches!(asset_server.get_load_state(handle), Some(LoadState::Loaded)) {
        if let Some(image) = images.get_mut(handle) {
            // Add the STORAGE_BINDING flag
            image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
            println!("Changed storage flag!");

            commands.insert_resource(ShaderData {
                tiles: material.1.tiles.clone()
            });
        }
    }
}

fn prepare_compute<'a>(
    mut commands: Commands,
    shader_data: Option<Res<ShaderData>>,
    render_device: Res<RenderDevice>,
    pipeline: Res<KernelPipeline>,
    bind_group: Option<Res<KernelBindGroup>>,
    mut param: (
        Res<'a, RenderAssets<GpuImage>>,
        Res<'a, FallbackImage>,
        Res<'a, RenderAssets<GpuShaderStorageBuffer>>
    ),
) {
    if bind_group.is_some() {return}

    let Some(shader_data) = shader_data else {
        println!("shader_data not yet available");
        return
    };
    let prepared_result = shader_data.as_bind_group(
        &pipeline.bind_group_layout,
        &render_device,
        &mut param,
    );
    if let Ok(prepared_numbers) = prepared_result {
        commands.insert_resource(KernelBindGroup(prepared_numbers.bind_group));
        println!("bind success!");
    } else {
        println!("retry next update");
        // we are retrying every frame regardless
    }
}

impl Node for DispatchKernel {
    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        // can't use because there is no view entity, uncommenting this line causes a hard-to-diagnose panic
        //let _view_entity = graph.view_entity();
        let kernel_pipeline = world.get_resource::<KernelPipeline>();
        let kernel_bind_group = world.get_resource::<KernelBindGroup>();
        let pipeline_cache = world.get_resource::<PipelineCache>();
        if let (Some(kernel_pipeline), Some(kernel_bind_group), Some(pipeline_cache)) = (kernel_pipeline, kernel_bind_group, pipeline_cache) {
            let mut pass = render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("Kernel Compute Pass"),
                    timestamp_writes: None,
                });
            if let Some(real_pipeline) = pipeline_cache.get_compute_pipeline(kernel_pipeline.pipeline) {
                println!("dispatch happening");
                pass.set_pipeline(&real_pipeline);
                pass.set_bind_group(0, &kernel_bind_group.0, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
        }
        Ok(())
    }
}
