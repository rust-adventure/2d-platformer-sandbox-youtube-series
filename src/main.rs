use bevy::{
    prelude::*, render::render_resource::TextureUsages,
    sprite::Anchor,
};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_ldtk::prelude::*;
use components::GroundDetection;
use gamepad::GamepadPlugin;
use heron::{Gravity, PhysicsPlugin};
use iyes_loopless::prelude::*;
// use map::{LdtkMap, LdtkMapBundle, LdtkPlugin};

// mod map;
mod components;
mod gamepad;
mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AssetState {
    Loading,
    Loaded,
}
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Playing,
}
fn main() {
    let mut app = App::new();
    AssetLoader::new(AssetState::Loading)
        .continue_to_state(AssetState::Loaded)
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app.add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(
            0.0, -2000., 0.0,
        )))
        .add_state(AssetState::Loading)
        .add_loopless_state(GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        // .add_plugin(TilemapPlugin)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(GroundDetection {
            on_ground: false,
        })
        // .register_ldtk_entity::<MyBundle>(
        //     "MyEntityIdentifier",
        // )
        // .add_system(set_texture_filters_to_nearest)
        .add_enter_system(GameState::Playing, setup)
        .add_system_set(
            SystemSet::on_enter(AssetState::Loaded)
                .with_system(move_to_loopless),
        )
        .add_system(
            systems::camera_fit_inside_current_level,
        )
        .add_system(systems::pause_physics_during_load)
        .add_system(systems::spawn_wall_collision)
        // .add_system(systems::movement)
        .add_system(systems::patrol)
        .add_system(
            systems::camera_fit_inside_current_level,
        )
        .add_system(systems::update_level_selection)
        .add_system(systems::ground_detection)
        .register_ldtk_entity::<components::PlayerBundle>(
            "Player",
        )
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .add_plugin(GamepadPlugin)
        .run();
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkAsset>,
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

fn move_to_loopless(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing))
}
fn setup(
    mut commands: Commands,
    images: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
) {
    let camera = OrthographicCameraBundle::new_2d();

    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: images.map.clone(),
        ..Default::default()
    });

    // let map_entity = commands.spawn().id();

    // commands.entity(map_entity).insert_bundle(
    //     LdtkMapBundle {
    //         ldtk_map: images.map.clone(),
    //         map: Map::new(0u16, map_entity),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..Default::default()
    //     },
    // );
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
