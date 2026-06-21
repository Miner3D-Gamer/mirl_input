impl ButtonState {
    #[must_use]
    /// Create a new button state -> Pressed, clicked, and released are calculated
    pub const fn new(current: bool, last: bool) -> Self {
        Self {
            down: current,
            clicked: current && !last,
            released: !current && last,
        }
    }
    /// Update the current state
    pub const fn update(&mut self, new: bool) {
        self.clicked = !self.down && new;
        self.released = self.down && !new;
        self.down = new;
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
/// The current state of a mouse buttons and if they have just been pressed
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ButtonState {
    pub down: bool,
    pub clicked: bool,
    pub released: bool,
}
