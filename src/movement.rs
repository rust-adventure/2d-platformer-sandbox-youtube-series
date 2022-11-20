use std::time::Duration;

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
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &Velocity,
    )>,
    time: Res<Time>,
    player_state: Res<PlayerState>,
) {
    for action_state in query_action_state.iter() {
        for (mut controller, output, velocity) in
            controllers.iter_mut()
        {
            match output.grounded {
                true => {
                    if action_state.just_pressed(
                        PlatformerAction::Jump,
                    ) {
                        controller.translation =
                            match controller.translation {
                                Some(mut v) => {
                                    v.y = 20.0;
                                    Some(v)
                                }
                                None => Some(Vec2::new(
                                    0.0, 20.0,
                                )),
                            };
                    } else {
                        controller.translation =
                            match controller.translation {
                                Some(mut v) => {
                                    v.y = -4.0;
                                    Some(v)
                                }
                                None => Some(Vec2::new(
                                    0.0, -4.0,
                                )),
                            };
                    }
                }
                false => {
                    if action_state.just_released(
                        PlatformerAction::Jump,
                    ) {
                        controller.translation =
                            match controller.translation {
                                Some(mut v) => {
                                    v.y = -8.0;
                                    Some(v)
                                }
                                None => Some(Vec2::new(
                                    0.0, -8.0,
                                )),
                            };
                    } else if action_state
                        .pressed(PlatformerAction::Jump)
                    {
                        let has_held_jump_for_duration =
                            action_state.current_duration(
                                PlatformerAction::Jump,
                            );
                        if has_held_jump_for_duration
                            >= Duration::from_secs(2)
                        {
                            controller.translation =
                                match controller.translation
                                {
                                    Some(mut v) => {
                                        v.y = -8.0;
                                        Some(v)
                                    }
                                    None => {
                                        Some(Vec2::new(
                                            0.0, -8.0,
                                        ))
                                    }
                                };
                        }
                    } else {
                    }
                }
            }
        }
    }
}

const TargetTopSpeed: f32 = 300.0;
// /// clamped_input is a 0.0-1.0 value representing the user's
// /// desired percentage of top speed to hold
// ///
// /// `current_velocity` is the current horizontal velocity
// fn calc_force_diff(
//     clamped_input: f32,
//     current_velocity: f32,
//     target_velocity: f32,
// ) -> f32 {
//     let target_speed = target_velocity * clamped_input;
//     let diff_to_make_up = target_speed - current_velocity;
//     let new_force = diff_to_make_up * 2.0;
//     new_force
// }
fn horizontal(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for action_state in query_action_state.iter() {
        for (mut controller, velocity) in
            controllers.iter_mut()
        {
            if action_state.pressed(PlatformerAction::Right)
            {
                let right_speed = action_state
                    .clamped_value(PlatformerAction::Right)
                    * 300.0
                    * time.delta_seconds();
                controller.translation = match controller
                    .translation
                {
                    Some(mut v) => {
                        v.x = right_speed;
                        Some(v)
                    }
                    None => {
                        Some(Vec2::new(right_speed, -4.0))
                    }
                }
            } else if action_state
                .pressed(PlatformerAction::Left)
            {
                let left_speed = -action_state
                    .clamped_value(PlatformerAction::Left)
                    * 300.0
                    * time.delta_seconds();
                controller.translation =
                    match controller.translation {
                        Some(mut v) => {
                            v.x = left_speed;
                            Some(v)
                        }
                        None => Some(Vec2::new(
                            left_speed, -4.0,
                        )),
                    }
            } else {
                controller.translation =
                    match controller.translation {
                        Some(mut v) => {
                            v.x = 0.0;
                            Some(v)
                        }
                        None => Some(Vec2::new(0.0, -4.0)),
                    };
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
