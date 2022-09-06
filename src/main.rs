use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};
use leafwing_input_manager::prelude::*;
use platformer::{
    actions::PlatformerAction,
    components::{self, GroundDetection},
    gamepad::GamepadPlugin,
    movement::MovementPlugin,
    systems, GameState,
};

fn main() {
    let mut app = App::new();
    app.add_loopless_state(GameState::AssetLoading);
    LoadingState::new(GameState::AssetLoading)
        // https://github.com/NiklasEi/bevy_asset_loader/issues/54
        .continue_to_state(GameState::Playing)
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app.add_plugins(DefaultPlugins)
        .add_plugin(ProgressPlugin::new(
            GameState::AssetLoading,
        ))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(
            InputManagerPlugin::<PlatformerAction>::default(
            ),
        )
        .insert_resource(LdtkSettings {
            level_spawn_behavior:
                LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
            set_clear_color:
                SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(GroundDetection {
            on_ground: false,
        })
        .add_enter_system(GameState::Playing, setup)
        .add_system(
            systems::camera_fit_inside_current_level,
        )
        // .add_system(systems::pause_physics_during_load)
        .add_system(systems::spawn_wall_collision)
        // .add_system(systems::movement)
        .add_system(systems::patrol)
        .add_system(systems::update_level_selection)
        .add_system(systems::ground_detection)
        .add_system(systems::spawn_ground_sensor)
        .register_ldtk_entity::<components::PlayerBundle>(
            "Player",
        )
        .add_system(systems::restart_level)
        .add_system(systems::player_added)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .add_plugin(GamepadPlugin)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            print_progress,
        )
        .run();
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "sandbox.ldtk")]
    map: Handle<LdtkAsset>,
}

fn setup(mut commands: Commands, images: Res<ImageAssets>) {
    // camera.orthographic_projection.scale = 2.;
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: images.map.clone(),
        ..Default::default()
    });
}

fn print_progress(progress: Option<Res<ProgressCounter>>) {
    if let Some(progress) = progress {
        info!(
            "Current progress: {:?}",
            progress.progress()
        );
    }
}
