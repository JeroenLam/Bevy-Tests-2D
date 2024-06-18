use bevy::prelude::*;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerInput>::default());
    }
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
pub enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Jump,
    Run,
    Action,
}

impl PlayerInput {
    pub fn player_one_keyboard() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            // ============ Keyboard ============
            // Arrow movement
            (PlayerInput::MoveUp, KeyCode::ArrowUp),
            (PlayerInput::MoveDown, KeyCode::ArrowDown),
            (PlayerInput::MoveLeft, KeyCode::ArrowLeft),
            (PlayerInput::MoveRight, KeyCode::ArrowRight),
            // WASD movement
            (PlayerInput::MoveUp, KeyCode::KeyW),
            (PlayerInput::MoveDown, KeyCode::KeyS),
            (PlayerInput::MoveLeft, KeyCode::KeyA),
            (PlayerInput::MoveRight, KeyCode::KeyD),
            // Jumping
            (PlayerInput::Jump, KeyCode::Space),
            // Running
            (PlayerInput::Run, KeyCode::ShiftLeft),
        ]);
        map
    }
    pub fn player_one_controller() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            // ============ Controller ============
            // Movement
            (PlayerInput::MoveLeft, GamepadButtonType::DPadLeft),
            (PlayerInput::MoveRight, GamepadButtonType::DPadRight),
            (PlayerInput::MoveUp, GamepadButtonType::DPadUp),
            (PlayerInput::MoveDown, GamepadButtonType::DPadDown),
            // Jumping
            (PlayerInput::Jump, GamepadButtonType::South),
            // Running
            (PlayerInput::Run, GamepadButtonType::East),
        ]);
        map
    }
}
