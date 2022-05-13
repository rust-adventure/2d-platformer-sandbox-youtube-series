use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_ldtk::prelude::*;
use components::GroundDetection;
use gamepad::GamepadPlugin;
use heron::{Gravity, PhysicsPlugin};
use iyes_loopless::prelude::*;

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
        .add_system(systems::spawn_ground_sensor)
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
}

fn move_to_loopless(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing))
}
fn setup(mut commands: Commands, images: Res<ImageAssets>) {
    let camera = OrthographicCameraBundle::new_2d();

    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: images.map.clone(),
        ..Default::default()
    });
}
