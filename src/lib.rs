pub mod actions;
pub mod components;
pub mod gamepad;
pub mod movement;
pub mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Playing,
}

// State Machine

// struct PlayerState<S> {
//     state: S,
// }
// impl Default for PlayerState<Idle> {
//     fn default() -> Self {
//         Self { state: Idle }
//     }
// }

// impl PlayerState<Idle> {
//     fn jump(&self) -> PlayerState<Jumping> {
//         PlayerState { state: Jumping }
//     }
// }

// impl PlayerState<Jumping> {
//     fn peak(&self) -> PlayerState<Falling> {
//         PlayerState { state: Falling }
//     }
// }
// struct Jumping;
// struct Falling;
// struct Idle;
