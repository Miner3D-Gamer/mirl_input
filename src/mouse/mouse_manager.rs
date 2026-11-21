use crate::mouse::{ButtonState, MouseButton, MouseButtonState};

#[mirl_macros::derive_all(zerocopy = false, eq = false, hash = false, default = true)]
/// A struct to manage the pressed mouse keys + scroll
#[allow(clippy::struct_excessive_bools)]
pub struct MouseState {
    /// The horizontal scroll
    pub scroll_x: f32,
    /// The vertical scroll
    pub scroll_y: f32,
    /// The pressed mouse buttons
    pub buttons: MouseButtonState,
}

impl MouseState {
    #[must_use]
    /// Create a default instance of [`MouseState`]
    pub const fn new() -> Self {
        Self::default()
    }
    /// Set the scroll
    pub const fn set_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x = xy.0;
        self.scroll_y = xy.1;
    }
    /// Add to the scroll
    pub fn add_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x += xy.0;
        self.scroll_y += xy.1;
    }
    /// Reset the scroll
    pub const fn reset_scroll(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
    }
    #[must_use]
    /// Get the mouse wheel scroll
    pub const fn get_scroll(&self) -> (f32, f32) {
        (self.scroll_x, self.scroll_y)
    }
    #[must_use]
    /// Check if a mouse button is pressed
    pub const fn get_mouse_button(&self, button: MouseButton) -> ButtonState {
        self.buttons.get_button(button)
    }
    /// Set the state of a mouse button
    pub const fn update_mouse_button(&mut self, button: MouseButton, value: bool) {
        self.buttons.update_mouse_button(button, value);
    }
}

// #[must_use]
// /// Get the value [`MouseButton`] of [`MouseState`]
// pub const fn map_button(button: MouseButton, mouse_manager: &MouseState) -> bool {
//     match button {
//         MouseButton::Left => mouse_manager.left_mouse_button,
//         MouseButton::Right => mouse_manager.right_mouse_button,
//         MouseButton::Middle => mouse_manager.middle_mouse_button,
//         MouseButton::Extra1 => mouse_manager.extra1_button,
//         MouseButton::Extra2 => mouse_manager.extra2_button,
//         MouseButton::Extra3 => mouse_manager.extra3_button,
//         MouseButton::Extra4 => mouse_manager.extra4_button,
//     }
// }
// /// Set the value [`MouseButton`] of [`MouseState`]
// pub const fn set_mouse_button(button: MouseButton, mouse_manager: &mut MouseState, value: bool) {
//     match button {
//         MouseButton::Left => mouse_manager.left_mouse_button = value,
//         MouseButton::Right => mouse_manager.right_mouse_button = value,
//         MouseButton::Middle => mouse_manager.middle_mouse_button = value,
//         MouseButton::Extra1 => mouse_manager.extra1_button = value,
//         MouseButton::Extra2 => mouse_manager.extra2_button = value,
//         MouseButton::Extra3 => mouse_manager.extra3_button = value,
//         MouseButton::Extra4 => mouse_manager.extra4_button = value,
//         MouseButton::Unsupported => (),
//     }
// }
