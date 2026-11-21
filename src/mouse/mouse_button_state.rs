use crate::mouse::MouseButton;

use super::{ButtonState, MouseSnapShot};

// TODO: Add `std::ops::Or` and `std::ops::Not`
#[mirl_macros::derive_all(default = true)]
#[allow(missing_docs)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseButtonState {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
    pub extra1: ButtonState,
    pub extra2: ButtonState,
    pub extra3: ButtonState,
    pub extra4: ButtonState,
    pub extra5: ButtonState,
    pub extra6: ButtonState,
}
impl MouseButtonState {
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new_default(left_down: bool, middle_down: bool, right_down: bool) -> Self {
        Self {
            left: ButtonState::new(left_down, false),
            middle: ButtonState::new(middle_down, false),
            right: ButtonState::new(right_down, false),
            extra1: ButtonState::default(),
            extra2: ButtonState::default(),
            extra3: ButtonState::default(),
            extra4: ButtonState::default(),
            extra5: ButtonState::default(),
            extra6: ButtonState::default(),
        }
    }
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new(left: ButtonState, middle: ButtonState, right: ButtonState) -> Self {
        Self {
            left,
            middle,
            right,
            extra1: ButtonState::default(),
            extra2: ButtonState::default(),
            extra3: ButtonState::default(),
            extra4: ButtonState::default(),
            extra5: ButtonState::default(),
            extra6: ButtonState::default(),
        }
    }
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new_advanced_default(
        // main: (bool, bool, bool),
        // normal_special: (bool, bool),
        // obscure_special: (bool, bool, bool),
        left_down: bool,
        middle_down: bool,
        right_down: bool,
        extra1: bool,
        extra2: bool,
        extra3: bool,
        extra4: bool,
        extra5: bool,
        extra6: bool,
    ) -> Self {
        Self {
            left: ButtonState::new(left_down, false),
            middle: ButtonState::new(middle_down, false),
            right: ButtonState::new(right_down, false),
            extra1: ButtonState::new(extra1, false),
            extra2: ButtonState::new(extra2, false),
            extra3: ButtonState::new(extra3, false),
            extra4: ButtonState::new(extra4, false),
            extra5: ButtonState::new(extra5, false),
            extra6: ButtonState::new(extra6, false),
        }
    }
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new_advanced(
        left: ButtonState,
        middle: ButtonState,
        right: ButtonState,
        extra1: ButtonState,
        extra2: ButtonState,
        extra3: ButtonState,
        extra4: ButtonState,
        extra5: ButtonState,
        extra6: ButtonState,
    ) -> Self {
        Self {
            left,
            middle,
            right,
            extra1,
            extra2,
            extra3,
            extra4,
            extra5,
            extra6,
        }
    }
    /// Updates the mouse button states
    pub const fn update(&mut self, left_down: bool, middle_down: bool, right_down: bool) {
        self.left.update(left_down);
        self.middle.update(middle_down);
        self.right.update(right_down);
    }
    // TODO: Add update_with_button_states_advanced
    /// Updates the mouse button states
    pub const fn update_with_button_states(
        &mut self,
        left_down: ButtonState,
        middle_down: ButtonState,
        right_down: ButtonState,
    ) {
        self.left = left_down;
        self.middle = middle_down;
        self.right = right_down;
    }
    #[allow(clippy::too_many_arguments)]
    /// Updates the mouse button states
    pub const fn update_advanced(
        &mut self,
        left_down: bool,
        middle_down: bool,
        right_down: bool,
        extra1: bool,
        extra2: bool,
        extra3: bool,
        extra4: bool,
    ) {
        self.left.update(left_down);
        self.middle.update(middle_down);
        self.right.update(right_down);
        self.extra1.update(extra1);
        self.extra2.update(extra2);
        self.extra3.update(extra3);
        self.extra4.update(extra4);
    }
    /// Updates the mouse button states using the current mouse snapshot
    pub const fn update_with_snapshot(&mut self, snapshot: &MouseSnapShot) {
        self.update_with_button_states(
            snapshot.left_down,
            snapshot.middle_down,
            snapshot.right_down,
        );
    }
    #[must_use]
    /// Get the value of [`MouseButton`]
    pub const fn get_button(&self, button: MouseButton) -> ButtonState {
        match button {
            MouseButton::Left => self.left,
            MouseButton::Right => self.right,
            MouseButton::Middle => self.middle,
            MouseButton::Extra1 => self.extra1,
            MouseButton::Extra2 => self.extra2,
            MouseButton::Extra3 => self.extra3,
            MouseButton::Extra4 => self.extra4,
            MouseButton::Extra5 => self.extra5,
            MouseButton::Extra6 => self.extra6,
        }
    }
    #[must_use]
    /// Get the value of [`MouseButton`]
    pub const fn get_button_mut(&mut self, button: MouseButton) -> &mut ButtonState {
        match button {
            MouseButton::Left => &mut self.left,
            MouseButton::Right => &mut self.right,
            MouseButton::Middle => &mut self.middle,
            MouseButton::Extra1 => &mut self.extra1,
            MouseButton::Extra2 => &mut self.extra2,
            MouseButton::Extra3 => &mut self.extra3,
            MouseButton::Extra4 => &mut self.extra4,
            MouseButton::Extra5 => &mut self.extra5,
            MouseButton::Extra6 => &mut self.extra6,
        }
    }
    /// Set the value of [`MouseButton`]
    pub const fn update_mouse_button(&mut self, button: MouseButton, value: bool) {
        self.get_button_mut(button).update(value);
    }
}
