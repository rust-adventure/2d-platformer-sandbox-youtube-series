use bevy::{
    prelude::*, render::render_resource::TextureUsages,
    sprite::Anchor,
};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_tilemap::{Map, TilemapPlugin};
use iyes_loopless::prelude::*;
use map::{LdtkMap, LdtkMapBundle, LdtkPlugin};

mod map;
fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::Playing)
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app.add_state(GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LdtkPlugin)
        .add_system(set_texture_filters_to_nearest)
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup),
        )
        .run();
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkMap>,
    #[asset(texture_atlas(
        tile_size_x = 80.,
        tile_size_y = 110.,
        columns = 9,
        rows = 3,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "zombie_tilesheet.png")]
    player: Handle<TextureAtlas>,
}

fn setup(mut commands: Commands, images: Res<ImageAssets>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform =
        Transform::from_scale(Vec3::new(5.0, 5.0, 1.0));
    commands.spawn_bundle(camera);

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(
        LdtkMapBundle {
            ldtk_map: images.map.clone(),
            map: Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    );
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(108.0, 108.0)),
            anchor: Anchor::CenterLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn use_my_assets(_image_assets: Res<ImageAssets>) {
    // do something using the asset handles from the resources
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Playing,
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) =
                    textures.get_mut(handle)
                {
                    texture.texture_descriptor.usage =
                        TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_SRC
                            | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
