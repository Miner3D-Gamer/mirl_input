pub mod keycodes;
// /// Mouse related items
// pub mod mouse;

pub use keycodes::KeyCode;

/// Keyboard manager, check/set if and when keys are pressed
pub mod keyboard_manager;
pub use keyboard_manager::KeyboardState;

/// A few lines of helper code for easier keybind handling time
pub mod keybinds;
