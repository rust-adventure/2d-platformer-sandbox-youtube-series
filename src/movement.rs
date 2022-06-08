use bevy::prelude::*;
use heron::Velocity;
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::*;

use crate::components::{Climber, GroundDetection, Player};
use crate::{actions::*, GameState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            jump.run_in_state(GameState::Playing),
        )
        // .add_system(Movement_input)
        .add_system(
            horizontal.run_in_state(GameState::Playing),
        );
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
            if action_state
                .just_pressed(PlatformerAction::Jump)
                && ground_detection.on_ground
            {
                velocity.linear.y = 900.;
                sprite.index = 1;
                // if let Some(_) = timer {
                //     commands
                //         .entity(entity)
                //         .remove::<AnimationTimer>();
                // }
            } else if ground_detection.on_ground {
                sprite.index = 0;
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
                velocity.linear.x = 300.;
                sprite.flip_x = false;
            } else if action_state
                .pressed(PlatformerAction::Left)
            {
                velocity.linear.x = -300.;
                sprite.flip_x = true;
            } else {
                velocity.linear.x = 0.;
            }
        }
    }
}
