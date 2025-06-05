use bevy::{
    asset::RenderAssetUsages, color::palettes::css::{ANTIQUE_WHITE, CRIMSON}, prelude::*, render::{
        camera::ScalingMode, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers
    }
};

use super::scene::MainCamera;

#[derive(Resource)]
pub struct TileAtlas(pub Handle<Image>);

#[derive(Component)]
pub struct TilesCamera;

pub(super) fn plugin(app: &mut App) {
    // Spawn the tileset camera.
    app.add_systems(Startup, (
        setup,
    ));
    app.add_systems(First, (
        copy_transform,
    ));
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 1024,
        height: 64,
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
    commands.insert_resource(TileAtlas(image_handle.clone()));

    commands.spawn((
        Name::new("Tilesheet Camera"),
        Camera3d::default(),
        Camera {
            target: image_handle.clone().into(),
            ..default()
        },
        TilesCamera,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            viewport_origin: vec2(0.0,0.0),
            scale: 1.0/32.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::default(),
        RenderLayers::layer(1),
    ));

    commands.spawn((
        ImageNode::new(
            image_handle.clone(),
        ),
        Node {
            width: Val::Px(size.width as f32),
            height: Val::Px(size.height as f32),
            ..default()
        },
        BackgroundColor(ANTIQUE_WHITE.into()),
        Outline::new(Val::Px(2.0), Val::ZERO, CRIMSON.into()),
    ));
}

fn copy_transform(main: Query<&Transform, With<MainCamera>>, mut tiles: Query<&mut Transform, (With::<TilesCamera>, Without::<MainCamera>)>) {
    let (Ok(main), Ok(tiles)) = (main.single(), tiles.single_mut()) else {return};
    // *tiles = Transform::from_rotation(rotation) *main.rotation;
}
