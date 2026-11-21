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

#[mirl_macros::derive_all(default = true)]
/// The current state of a mouse buttons and if they have just been pressed
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ButtonState {
    pub down: bool,
    pub clicked: bool,
    pub released: bool,
}

const impl std::ops::BitOr for ButtonState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            down: self.down || rhs.down,
            clicked: self.clicked || rhs.clicked,
            released: self.released || rhs.released,
        }
    }
}

const impl std::ops::BitAnd for ButtonState {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            down: self.down && rhs.down,
            clicked: self.clicked && rhs.clicked,
            released: self.released && rhs.released,
        }
    }
}
