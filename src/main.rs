use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_ldtk::prelude::*;
use heron::{Gravity, PhysicsPlugin, Velocity};
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};
use leafwing_input_manager::prelude::*;
use platformer::{
    actions::PlatformerAction,
    components::{self, Climber, GroundDetection, Player},
    gamepad::{AnimationTimer, GamepadPlugin},
    movement::MovementPlugin,
    systems, GameState,
};

fn main() {
    let mut app = App::new();
    app.add_loopless_state(GameState::AssetLoading);
    AssetLoader::new(GameState::AssetLoading)
        // https://github.com/NiklasEi/bevy_asset_loader/issues/54
        .continue_to_state(GameState::Playing)
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app.add_plugins(DefaultPlugins)
        .add_plugin(ProgressPlugin::new(
            GameState::AssetLoading,
        ))
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(
            0.0, -2000., 0.0,
        )))
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
    let camera = OrthographicCameraBundle::new_2d();

    commands.spawn_bundle(camera);

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
