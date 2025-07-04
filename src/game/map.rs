use std::borrow::Cow;

use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        graph::CameraDriverLabel,
        mesh::PrimitiveTopology,
        render_asset::RenderAssets,
        render_graph::{Node, RenderGraph, RenderLabel},
        render_resource::{AsBindGroup, BindGroup, BindGroupLayout, CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor, Extent3d, Origin3d, PipelineCache, ShaderRef, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
        renderer::{RenderDevice, RenderQueue},
        storage::GpuShaderStorageBuffer,
        texture::{FallbackImage, GpuImage},
        view::NoFrustumCulling,
        Render, RenderApp, RenderSet
    }
};
use rand::Rng;

use crate::screens::Screen;

use super::prelude::*;

pub(super) struct MapPlugin;

#[derive(RenderLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MyRenderLabels {
    Simulate,
}

#[derive(Component)]
pub struct TileMap;

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
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            // Add code for updating clicked tiles.
            render_app.add_systems(Render, handle_click.in_set(RenderSet::Queue));

            // Inject the compute kernel.
            render_app.init_resource::<KernelPipeline>();
            render_app.add_systems(Render, prepare_compute.in_set(RenderSet::Prepare));

            // Add our compute kernel to the render graph
            let mut render_graph = render_app.world_mut().get_resource_mut::<RenderGraph>().expect("Should be able to get render graph");
            let kernel_node = DispatchKernel{};
            render_graph.add_node(MyRenderLabels::Simulate, kernel_node);

            // Set the kernel to run before the main pass.
            let r = render_graph.try_add_node_edge(
                MyRenderLabels::Simulate,
                CameraDriverLabel,
            );
            if r.is_err() {
                println!("{:?}", r);
            }
            println!("KernelPipeline added");
        }
    }
}

/**
 * Shader for drawing the tilemap.
 */
#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct TilemapMaterial {
    #[storage_texture(0, image_format=Rgba8Uint, visibility(vertex,fragment), access=ReadOnly)] map: Handle<Image>,
    #[texture(1)] #[sampler(2)] tileset: Handle<Image>,
    #[uniform(3)] hover_tile: Vec4,
    #[uniform(4)] tile_size: f32,
    #[uniform(5)] tile_count: f32,
    #[uniform(6)] selected_tile: UVec2,
}

#[derive(TypePath,AsBindGroup,Resource,Clone,ExtractResource)]
struct ShaderData {
    #[storage_texture(0, image_format=Rgba8Uint)] tiles: Handle<Image>,
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
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    tileset: Res<Tileset>,
) {
    // Fullscreen triangle (covers full screen)
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        // triangle that covers full screen in clip space
        [-1.0, -3.0, 0.0],
        [ 3.0,  1.0, 0.0],
        [-1.0,  1.0, 0.0],
    ]);

    let size = Extent3d {
        width: MAP_SIZE,
        height: MAP_SIZE,
        ..default()
    };

    let mut map_data = Vec::<u8>::with_capacity((MAP_SIZE*MAP_SIZE*4) as usize);
    for _ in 0..MAP_SIZE * MAP_SIZE {
        map_data.push(rand::thread_rng().gen_range(0..44));
        map_data.push(rand::thread_rng().gen_range(0..6));
        // Init xorshift16 with non-zero seed.
        let prng: u32 = rand::thread_rng().gen_range(1..65536);
        map_data.push((prng / 256) as u8);
        map_data.push((prng % 256) as u8);
    }

    // This is the texture that will be rendered to.
    let map_image = Image {
        data: Some(map_data),
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            format: TextureFormat::Rgba8Uint,
            dimension: TextureDimension::D2,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        texture_view_descriptor: None,
        asset_usage: RenderAssetUsages::RENDER_WORLD,
    };

    let map_handle = images.add(map_image);
    commands.insert_resource(ShaderData {
        tiles: map_handle.clone()
    });

    commands.spawn((
        Name::new("Tilemap"),
        TileMap,
        NoFrustumCulling,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(TilemapMaterial{
            map: map_handle,
            tileset: tileset.0.clone(),
            hover_tile: Vec4::ZERO,
            tile_size: TILE_SIZE as f32,
            tile_count: TILE_COUNT as f32,
            selected_tile: UVec2::ZERO,
        })),
        Transform::IDENTITY,
    )).observe(|trigger: Trigger<Pointer<Move>>, mut mouse_pos: ResMut<MousePos>|{
        mouse_pos.on_screen = true;
        match mouse_pos.click_started {
            Some(start) if trigger.pointer_location.position.distance_squared(start) > 10.0 => mouse_pos.click_started = None,
            _ => {}
        }

        // Update hovered hexagon
        let hex = POSITION_TO_CUBE * trigger.event().hit.position.unwrap();
        mouse_pos.hex_cell  = round_hex(hex).as_ivec3();
    }).observe(|_trigger: Trigger<Pointer<Out>>, mut mouse_pos: ResMut<MousePos>|{
        mouse_pos.on_screen = false;
        mouse_pos.click_started = None;
    }).observe(|trigger: Trigger<Pointer<Pressed>>, mut mouse_pos: ResMut<MousePos>|{
        if trigger.button == PointerButton::Primary {
            mouse_pos.click_started = Some(trigger.pointer_location.position);
        }
    }).observe(|trigger: Trigger<Pointer<Released>>, mut mouse_pos: ResMut<MousePos>|{
        match mouse_pos.click_started {
            Some(start) if trigger.pointer_location.position.distance_squared(start) <= 10.0 && trigger.button == PointerButton::Primary => {
                mouse_pos.click = true;
                mouse_pos.click_started = None;
            }
            _ => {}
        }
    });
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

fn update_tile(mouse: Res<MousePos>, mut materials: ResMut<Assets<TilemapMaterial>>) {
    let tile = mouse.hex_cell.as_vec3();
    for mat in materials.iter_mut() {
        mat.1.hover_tile = tile.extend(
            if mouse.on_screen {0.0} else {1.0}
        );
        mat.1.selected_tile = mouse.selected_tile;
    }
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
        let bind_group_layout = ShaderData::bind_group_layout(render_device);
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
                //println!("dispatch happening");
                pass.set_pipeline(real_pipeline);
                pass.set_bind_group(0, &kernel_bind_group.0, &[]);
                pass.dispatch_workgroups(16, 16, 1);
            }
        }
        Ok(())
    }
}

fn handle_click(
    queue: Res<RenderQueue>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    updates: Res<MousePos>,
    shader_data: Option<Res<ShaderData>>,
) {
    if !updates.click {return} // Bail out if there was no click.

    // Find the necessary resources
    let Some(shader_data) = shader_data else {return};
    let Some(image) = gpu_images.get(shader_data.tiles.id()) else {return};

    // Prepare data
    let prng: u32 = rand::thread_rng().gen_range(1..65536);
    let map_data = [
        updates.selected_tile.x as u8,
        updates.selected_tile.y as u8,
        // Init xorshift16 with non-zero seed.
        (prng / 256) as u8,
        (prng % 256) as u8,
    ];

    // Queue the pixel write
    queue.write_texture(
        TexelCopyTextureInfo{
            texture: &image.texture,
            mip_level: 0,
            origin: Origin3d {
                x: (updates.hex_cell.x & 1023) as u32,
                y: (updates.hex_cell.y & 1023) as u32,
                z: 0,
            },
            aspect: TextureAspect::All,
        },
        &map_data,
        TexelCopyBufferLayout{
            offset: 0,
            bytes_per_row: Some(4),
            rows_per_image: Some(1),
        },
        Extent3d{
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }
    );
}
