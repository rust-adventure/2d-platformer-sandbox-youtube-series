use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlatformerAction {
    Right,
    Left,
    Down,
    Up,

    Jump,
    Heal,
    Dash,
    Pause,
    Menus,
}
