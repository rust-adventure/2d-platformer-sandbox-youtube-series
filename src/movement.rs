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
            // &mut TextureAtlasSprite,
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
            // mut sprite,
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

                // sprite.index = 1;
                // if let Some(_) = timer {
                //     commands
                //         .entity(entity)
                //         .remove::<AnimationTimer>();
                // }
            } else if ground_detection.on_ground {
                // sprite.index = 0;
                *gravity_scale = GravityScale(1.0);
            } else {
            }
        }
    }
}

const TargetTopSpeed: f32 = 300.0;
/// clamped_input is a 0.0-1.0 value representing the user's
/// desired percentage of top speed to hold
///
/// `current_velocity` is the current horizontal velocity
fn calc_force_diff(
    clamped_input: f32,
    current_velocity: f32,
    target_velocity: f32,
) -> f32 {
    let target_speed = target_velocity * clamped_input;
    let diff_to_make_up = target_speed - current_velocity;
    let new_force = diff_to_make_up * 2.0;
    new_force
}
fn horizontal(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    // axes: Res<Axis<GamepadAxis>>,
    mut query_player: Query<
        (
            Entity,
            &mut Velocity,
            &mut ExternalForce,
            // &mut TextureAtlasSprite,
            // Option<&AnimationTimer>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for action_state in query_action_state.iter() {
        for (
            entity,
            mut velocity,
            mut force,
            // mut sprite,
            // timer,
        ) in query_player.iter_mut()
        {
            if action_state.pressed(PlatformerAction::Right)
            {
                let new_horizontal_force = calc_force_diff(
                    action_state.clamped_value(
                        PlatformerAction::Right,
                    ),
                    velocity.linvel.x,
                    TargetTopSpeed,
                );

                force.force.x = new_horizontal_force;
                // sprite.flip_x = false;
            } else if action_state
                .pressed(PlatformerAction::Left)
            {
                let new_horizontal_force = calc_force_diff(
                    action_state.clamped_value(
                        PlatformerAction::Left,
                    ),
                    velocity.linvel.x,
                    -TargetTopSpeed,
                );

                force.force.x = new_horizontal_force;
                // sprite.flip_x = true;
            } else {
                if velocity.linvel.x.abs() > 0.01 {
                    let new_horizontal_force =
                        -velocity.linvel.x;

                    force.force.x = new_horizontal_force;
                }
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
