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

use crate::theme::palette;

use super::prelude::*;

const TILESET_PREVIEW: bool = false;

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

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

    if TILESET_PREVIEW {
        commands.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        }).with_children(|parent| {
            parent.spawn((
                ImageNode::new(
                    image_handle.clone(),
                ),
                Node {
                    width: Val::Px(size.width as f32),
                    height: Val::Px(size.height as f32),
                    ..default()
                },
                BackgroundColor(palette::BUTTON_HOVERED_BACKGROUND),
                Outline::new(Val::Px(2.0), Val::ZERO, palette::BUTTON_PRESSED_BACKGROUND),
            ));
        });
    }
}

fn copy_transform(main: Query<&MainCamera>, mut tiles: Query<(&mut Transform, &Tile)>) {
    let Ok(main) = main.single() else {return};
    let base = main.next_transform.rotation.inverse();
    for (mut transform, tile) in tiles.iter_mut() {
        transform.rotation = base * tile.rotation;
    }
}
