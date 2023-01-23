use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::*;
use statig::{
    prelude::*, InitializedStatemachine, StateOrSuperstate,
};
use std::time::Duration;

// use crate::components::{Climber,
// GroundDetection, Player};
use crate::{actions::*, GameState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            jump.run_in_state(GameState::Playing),
        )
        // // .add_system(Movement_input)
        .add_system(
            horizontal.run_in_state(GameState::Playing),
        )
        .add_system(fall.run_in_state(GameState::Playing))
        .add_system(machine_events)
        .add_system(debug_actions);
    }
}

#[derive(Default)]
struct PlayerStateMachine {
    last_jump: Option<Duration>,
}
#[derive(Debug)]
pub enum Event {
    Jump { event_time: Duration },
    Heal,
    Crouch,
    Land,
    Fall,
}

#[derive(Component)]
pub struct PlayerState(
    InitializedStatemachine<PlayerStateMachine>,
);
impl Default for PlayerState {
    fn default() -> Self {
        Self(
            PlayerStateMachine::default()
                .state_machine()
                .init(),
        )
    }
}

#[state_machine(
    initial = "State::idle()",
    on_dispatch = "Self::on_dispatch",
    on_transition = "Self::on_transition",
    state(derive(Debug)),
    superstate(derive(Debug))
)]
impl PlayerStateMachine {
    fn on_transition(
        &mut self,
        source: &State,
        target: &State,
    ) {
        info!(
            "transitioned from `{:?}` to `{:?}`",
            source, target
        );
    }

    fn on_dispatch(
        &mut self,
        state: StateOrSuperstate<PlayerStateMachine>,
        event: &Event,
    ) {
        info!(
            "dispatched `{:?}` to `{:?}`",
            event, state
        );
    }
    #[state]
    fn idle(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                self.last_jump = Some(*event_time);
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn jumping(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn healing(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn crouching(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
    #[state]
    fn falling(event: &Event) -> Response<State> {
        match event {
            Event::Jump { event_time } => {
                Transition(State::jumping())
            }
            Event::Heal => Transition(State::healing()),
            Event::Crouch => Transition(State::crouching()),
            Event::Land => Transition(State::idle()),
            Event::Fall => Transition(State::falling()),
        }
    }
}

fn machine_events(
    query_action_state: Query<
        &ActionState<PlatformerAction>,
    >,
    mut commands: Commands,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &Velocity,
        &mut PlayerState,
    )>,
    time: Res<Time>,
) {
    for (_, _output, _, mut state_machine) in
        &mut controllers
    {
        match state_machine.0.state() {
            // State::Idle {  } => todo!(),
            State::Jumping {} => {
                if let Some(last_jump) =
                    state_machine.0.last_jump
                {
                    if (time.elapsed() - last_jump)
                        > Duration::from_millis(500)
                    {
                        state_machine
                            .0
                            .handle(&Event::Fall);
                    }
                }
            }
            // State::Crouching {  } => todo!(),
            // State::Falling {  } => todo!(),
            // State::Healing {  } => todo!(),
            _ => {}
        }
    }
    for action_state in &query_action_state {
        for (_, output, _, mut state_machine) in
            &mut controllers
        {
            match state_machine.0.state() {
                State::Idle {} => {
                    // info!("idling");
                    if action_state.just_pressed(
                        PlatformerAction::Jump,
                    ) {
                        state_machine.0.handle(
                            &Event::Jump {
                                event_time: time.elapsed(),
                            },
                        );
                    }
                }
                State::Jumping {} => {
                    // info!("jumping");
                    if let Some(last_jump) =
                        state_machine.0.last_jump
                    {
                        if output.grounded
                            && 
                            // systems can run fast enough that the newly jumping
                            // player can still be in their original pre-takeoff contact
                            // with the ground
                            //
                            // maybe replace with "last_left_ground" field?
                            time.elapsed() - last_jump
                                > Duration::from_millis(50)
                        {
                            state_machine
                                .0
                                .handle(&Event::Land);
                        }
                    }
                }
                State::Crouching {} => {
                    // info!("crouching");
                }
                State::Healing {} => {
                    // info!("healing");
                }
                State::Falling {} => {
                    // info!("falling");
                }
            }
        }
    }
}
fn fall(
    mut commands: Commands,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &Velocity,
        &mut PlayerState,
        &ActionState<PlatformerAction>,
    )>,
    time: Res<Time>,
) {
    for (
        mut controller,
        output,
        velocity,
        mut state_machine,
        action_state,
    ) in &mut controllers
    {
        if let State::Falling {} = state_machine.0.state() {
            if output.grounded {
                state_machine.0.handle(&Event::Land);
            } else {
                controller.translation =
                    match controller.translation {
                        Some(mut v) => {
                            v.y = -20.0;
                            Some(v)
                        }
                        None => Some(Vec2::new(0.0, -20.0)),
                    };
            }
        }
    }
}
fn jump(
    mut commands: Commands,
    mut controllers: Query<(
        &mut KinematicCharacterController,
        // &KinematicCharacterControllerOutput,
        &Velocity,
        &mut PlayerState,
        &ActionState<PlatformerAction>,
    )>,
    time: Res<Time>,
) {
    for (
        mut controller,
        // output,
        velocity,
        mut state_machine,
        action_state,
    ) in &mut controllers
    {
        if action_state
            .just_released(PlatformerAction::Jump)
        {
            state_machine.0.handle(&Event::Fall);
        } else if let State::Jumping {} =
            state_machine.0.state()
        {
            controller.translation =
                match controller.translation {
                    Some(mut v) => {
                        v.y = 10.0;
                        Some(v)
                    }
                    None => Some(Vec2::new(0.0, 10.0)),
                };
        }
    }
}

const TargetTopSpeed: f32 = 300.0;
// /// clamped_input is a 0.0-1.0 value
// representing the user's /// desired percentage
// of top speed to hold ///
// /// `current_velocity` is the current
// horizontal velocity fn calc_force_diff(
//     clamped_input: f32,
//     current_velocity: f32,
//     target_velocity: f32,
// ) -> f32 {
//     let target_speed = target_velocity *
// clamped_input;     let diff_to_make_up =
// target_speed - current_velocity;
//     let new_force = diff_to_make_up * 2.0;
//     new_force
// }
fn horizontal(
    mut controllers: Query<(
        &ActionState<PlatformerAction>,
        &mut KinematicCharacterController,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for (action_state, mut controller, velocity) in
        controllers.iter_mut()
    {
        let value = if action_state
            .pressed(PlatformerAction::Horizontal)
        {
            action_state
                .action_data(PlatformerAction::Horizontal)
                .value
        } else if action_state
            .pressed(PlatformerAction::Right)
        {
            action_state
                .clamped_value(PlatformerAction::Right)
        } else if action_state
            .pressed(PlatformerAction::Left)
        {
            -action_state
                .clamped_value(PlatformerAction::Left)
        } else {
            0.0
        };

        let value = value * 300.0 * time.delta_seconds();
        controller.translation =
            match controller.translation {
                Some(mut v) => {
                    v.x = value;
                    Some(v)
                }
                None => Some(Vec2::new(value, -4.0)),
            };
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
