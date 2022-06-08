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
