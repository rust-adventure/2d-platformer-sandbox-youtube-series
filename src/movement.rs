use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::*;

use crate::components::{Climber, GroundDetection, Player};
use crate::{actions::*, GameState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .add_system(
                jump.run_in_state(GameState::Playing),
            )
            // // .add_system(Movement_input)
            .add_system(
                horizontal.run_in_state(GameState::Playing),
            )
            .add_system(debug_actions);
    }
}

enum PlayerState {
    Idle,
    Jumping,
    Falling,
}
impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

fn jump(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    mut commands: Commands,
    // axes: Res<Axis<GamepadAxis>>,
    // buttons: Res<Input<GamepadButton>>,
    // my_gamepad: Option<Res<MyGamepad>>,
    mut query_player: Query<
        (
            Entity,
            &mut Velocity,
            &mut Climber,
            &mut TextureAtlasSprite,
            // Option<&AnimationTimer>,
            &mut GravityScale,
            &GroundDetection,
        ),
        With<Player>,
    >,
    player_state: Res<PlayerState>,
    // mut gravity: ResMut<Gravity>,
    time: Res<Time>,
) {
    for action_state in query_action_state.iter() {
        for (
            entity,
            mut velocity,
            mut climber,
            mut sprite,
            // timer,
            mut gravity_scale,
            ground_detection,
        ) in query_player.iter_mut()
        {
            if action_state
                .just_released(PlatformerAction::Jump)
            {
                *gravity_scale = GravityScale(30.0);
            } else if action_state
                .just_pressed(PlatformerAction::Jump)
                && ground_detection.on_ground
            {
                velocity.linvel.y = 200.;

                sprite.index = 1;
                // if let Some(_) = timer {
                //     commands
                //         .entity(entity)
                //         .remove::<AnimationTimer>();
                // }
            } else if ground_detection.on_ground {
                sprite.index = 0;
                *gravity_scale = GravityScale(1.0);
            } else {
            }
        }
    }
}

fn horizontal(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    mut commands: Commands,
    // axes: Res<Axis<GamepadAxis>>,
    mut query_player: Query<
        (
            Entity,
            &mut Velocity,
            &mut Climber,
            &mut TextureAtlasSprite,
            // Option<&AnimationTimer>,
            &GroundDetection,
        ),
        With<Player>,
    >,
) {
    for action_state in query_action_state.iter() {
        for (
            entity,
            mut velocity,
            mut climber,
            mut sprite,
            // timer,
            ground_detection,
        ) in query_player.iter_mut()
        {
            if action_state.pressed(PlatformerAction::Right)
            {
                velocity.linvel.x = 300.;
                sprite.flip_x = false;
            } else if action_state
                .pressed(PlatformerAction::Left)
            {
                velocity.linvel.x = -300.;
                sprite.flip_x = true;
            } else if action_state
                .pressed(PlatformerAction::Horizontal)
            {
                let move_value = action_state
                    .value(PlatformerAction::Horizontal);
                if move_value == 0.0 {
                    velocity.linvel.x = 0.;
                } else if move_value.signum() == 1.0 {
                    velocity.linvel.x = 300.;
                    sprite.flip_x = false;
                } else if move_value.signum() == -1.0 {
                    velocity.linvel.x = -300.;
                    sprite.flip_x = true;
                } else {
                    error!(
                        "unexpected move_value {}",
                        move_value
                    );
                }
            } else {
                velocity.linvel.x = 0.;
            }
        }
    }
}

fn debug_actions(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
) {
    for action in query_action_state.iter() {
        for aaction in action
            .get_pressed()
            .iter()
            .filter(|v| v != &&PlatformerAction::Horizontal)
        {
            // dbg!(aaction);
            // dbg!(action.action_data(*aaction));
        }
    }
}
