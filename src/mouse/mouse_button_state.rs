use super::{ButtonState, MouseSnapShot};

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseButtonState {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
}
impl MouseButtonState {
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new_default(left_down: bool, middle_down: bool, right_down: bool) -> Self {
        Self {
            left: ButtonState::new(left_down, false),
            middle: ButtonState::new(middle_down, false),
            right: ButtonState::new(right_down, false),
        }
    }
    #[must_use]
    /// Create a new button state using the current mouse state, assumes that the buttons have been released last frame
    pub const fn new(
        left_down: bool,
        middle_down: bool,
        right_down: bool,
        previous_left_down: bool,
        previous_middle_down: bool,
        previous_right_down: bool,
    ) -> Self {
        Self {
            left: ButtonState::new(left_down, previous_left_down),
            middle: ButtonState::new(middle_down, previous_middle_down),
            right: ButtonState::new(right_down, previous_right_down),
        }
    }
    /// Updates the mouse button states
    pub const fn update(&mut self, left_down: bool, middle_down: bool, right_down: bool) {
        self.left.update(left_down);
        self.middle.update(middle_down);
        self.right.update(right_down);
    }
    /// Updates the mouse button states using the current mouse snapshot
    pub const fn update_with_snapshot(&mut self, snapshot: &MouseSnapShot) {
        self.update(
            snapshot.left_down,
            snapshot.middle_down,
            snapshot.right_down,
        );
    }
}
