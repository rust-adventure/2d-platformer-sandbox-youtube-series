use leafwing_input_manager::Actionlike;

#[derive(
    Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug,
)]
pub enum PlatformerAction {
    Right,
    Left,
    Down,
    Up,

    Horizontal,

    Jump,
    Heal,
    Dash,
    Pause,
    Menus,
}
