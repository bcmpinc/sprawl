use std::f32::consts::PI;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        camera::ScalingMode,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers
    }
};
use widget::button_small;

use crate::theme::prelude::*;

use super::prelude::*;

#[derive(Resource)]
pub struct Tileset(pub Handle<Image>);

#[derive(Component)]
pub struct TilesCamera;

#[derive(Component, Default)]
pub struct Tile {
    rotation: Quat,
}

impl Tile {
    pub fn rotated(rotation: i32) -> Self {
        Self {
            rotation: Quat::from_rotation_y(PI * rotation as f32 / 3.0)
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    // Spawn the tileset camera.
    app.add_systems(Startup, (
        setup,
    ));
    app.add_systems(Update, (
        copy_transform,
    ));
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut atlasses: ResMut<Assets<TextureAtlasLayout>>,
) {
    let size = Extent3d {
        width: TILE_SIZE * TILE_COUNT,
        height: TILE_SIZE * 6,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // Save tileset handle in a resource
    commands.insert_resource(Tileset(image_handle.clone()));

    commands.spawn((
        Name::new("Tilesheet Camera"),
        Camera3d::default(),
        Camera {
            target: image_handle.clone().into(),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        TilesCamera,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            viewport_origin: vec2(0.0,0.0),
            scale: 2.0/TILE_SIZE as f32,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
        RenderLayers::layer(1),
    ));

    let atlas = TextureAtlasLayout::from_grid(uvec2(TILE_SIZE, TILE_SIZE), TILE_COUNT, 6, None, None);
    let layout = atlasses.add(atlas);

    commands.spawn((
        Name::new("GUI Container"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        Pickable::IGNORE,
    )).with_children(|parent| {
        parent.spawn((
            Name::new("Tileset preview"),
            ImageNode::from_atlas_image(
                image_handle.clone(),
                TextureAtlas {
                    layout,
                    index: 0
                }
            ),
            Node {
                width: Val::Px(TILE_SIZE as f32),
                height: Val::Px(TILE_SIZE as f32),
                ..default()
            },
            BackgroundColor(ui_palette::BUTTON_BACKGROUND),
            Button,
            ui_palette::BUTTON_INTERACTION_PALETTE,
            BorderRadius::all(Val::Px(30.0)),
        )).observe(|trigger: Trigger<Pointer<Click>>, mut mouse_pos: ResMut<MousePos>| {
            let dir = Vec2::ONE - 2.0 * trigger.hit.position.unwrap().xy();
            if dir.x < -dir.y.abs() {
                mouse_pos.selected_tile.y -= 1;
            }
            if dir.x >  dir.y.abs() {
                mouse_pos.selected_tile.y += 1;
            }
            if dir.y < -dir.x.abs() {
                mouse_pos.selected_tile.x -= 1;
            }
            if dir.y >  dir.x.abs() {
                mouse_pos.selected_tile.x += 1;
            }
        });
    });
}

fn copy_transform(main: Query<&Transform, With<MainCamera>>, mut tiles: Query<(&mut Transform, &Tile), Without<MainCamera>>) {
    let Ok(main) = main.single() else {return};
    let base = main.rotation.inverse();
    for (mut transform, tile) in tiles.iter_mut() {
        transform.rotation = base * tile.rotation;
    }
}
