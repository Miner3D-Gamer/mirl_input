use crate::mouse::ButtonState;

// use super::MouseButtonState;

#[mirl_macros::derive_all(zerocopy = false, default = true, hash = false, eq = false)]
/// A data storage helper holding the current state of the mouse
pub struct MouseSnapShot {
    /// The current position of the mouse
    pub position: Option<(f32, f32)>,
    /// The current mouse scroll
    pub scroll: (f32, f32),
    /// If the left mouse button is down
    pub left_down: ButtonState,
    /// If the right mouse button is down
    pub middle_down: ButtonState,
    /// If the right mouse button is down
    pub right_down: ButtonState,
}
// impl MouseSnapShot {
//     #[must_use]
//     /// Convert the current snapshot into a proper
//     pub const fn to_mouse_button_state(
//         &self,
//         previous_left: bool,
//         previous_middle: bool,
//         previous_right: bool,
//     ) -> MouseButtonState {
//         MouseButtonState::new(
//             self.left_down,
//             self.middle_down,
//             self.right_down,
//             previous_left,
//             previous_middle,
//             previous_right,
//         )
//     }
// }
impl MouseSnapShot {
    #[must_use]
    /// Sets the current mouse position
    pub const fn set_position(mut self, position: Option<(f32, f32)>) -> Self {
        self.position = position;
        self
    }
    #[must_use]
    /// Sets the current mouse scroll
    pub const fn set_scroll(mut self, scroll: (f32, f32)) -> Self {
        self.scroll = scroll;
        self
    }
    #[must_use]
    /// Sets whether the left mouse button is down
    pub const fn update_left_down(mut self, left_down: ButtonState) -> Self {
        self.left_down = left_down;
        self
    }
    #[must_use]
    /// Sets whether the middle mouse button is down
    pub const fn set_middle_down(mut self, middle_down: ButtonState) -> Self {
        self.middle_down = middle_down;
        self
    }
    #[must_use]
    /// Sets whether the right mouse button is down
    pub const fn set_right_down(mut self, right_down: ButtonState) -> Self {
        self.right_down = right_down;
        self
    }
}
