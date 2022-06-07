use bevy::prelude::*;
use heron::Velocity;

use crate::components::{Climber, GroundDetection, Player};

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections)
            .add_system(gamepad_input)
            .add_system(animate_sprite);
    }
}
/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Debug)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!(
                    "New gamepad connected with ID: {:?}",
                    id
                );

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands
                        .insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!(
                    "Lost gamepad connection with ID: {:?}",
                    id
                );

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) =
                    my_gamepad.as_deref()
                {
                    if old_id == id {
                        commands
                            .remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn gamepad_input(
    mut commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut Climber,
            &mut TextureAtlasSprite,
            Option<&AnimationTimer>,
            &GroundDetection,
        ),
        With<Player>,
    >,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };
    // dbg!(&gamepad);

    for (
        entity,
        mut velocity,
        mut climber,
        mut sprite,
        timer,
        ground_detection,
    ) in query.iter_mut()
    {
        // The joysticks are represented using a separate axis for X and Y

        let axis_lx = GamepadAxis(
            gamepad,
            GamepadAxisType::LeftStickX,
        );
        let axis_ly = GamepadAxis(
            gamepad,
            GamepadAxisType::LeftStickY,
        );

        if let (Some(x), Some(y)) =
            (axes.get(axis_lx), axes.get(axis_ly))
        {
            // combine X and Y into one vector
            let left_stick_pos = Vec2::new(x, y);
            if left_stick_pos.x != 0.0 {
                match left_stick_pos.x.signum() {
                    -1.0 => {
                        sprite.flip_x = true;
                    }
                    1.0 => {
                        sprite.flip_x = false;
                    }
                    _ => {}
                };
            };
            velocity.linear.x = left_stick_pos.x * 300.;

            // + left_stick_pos.x.signum() * 100.0;
            // dbg!(velocity.linear.x, left_stick_pos.x);

            if (velocity.linear.x.abs() > 0.0)
                && timer.is_none()
                && ground_detection.on_ground
            {
                commands.entity(entity).insert(
                    AnimationTimer(Timer::from_seconds(
                        0.1, true,
                    )),
                );
            } else if !(velocity.linear.x.abs() > 0.0) {
                if let Some(_) = timer {
                    commands
                        .entity(entity)
                        .remove::<AnimationTimer>();
                }
            }
            // // Example: check if the stick is pushed up
            // if left_stick_pos.length() > 0.9
            //     && left_stick_pos.y > 0.5
            // {
            //     // do something
            //     dbg!("here");
            // }
        }

        // In a real game, the buttons would be configurable, but here we hardcode them
        let jump_button = GamepadButton(
            gamepad,
            GamepadButtonType::South,
        );
        let heal_button =
            GamepadButton(gamepad, GamepadButtonType::East);

        if buttons.just_pressed(jump_button)
            && ground_detection.on_ground
        {
            velocity.linear.y = 900.;
            sprite.index = 1;
            if let Some(_) = timer {
                commands
                    .entity(entity)
                    .remove::<AnimationTimer>();
            }
        } else if ground_detection.on_ground {
            sprite.index = 0;
        }

        if buttons.pressed(heal_button) {
            // button being held down: heal the player
            dbg!("circle");
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = match sprite.index {
                9 => 10,
                10 => 9,
                _ => 9,
            }
        }
    }
}
