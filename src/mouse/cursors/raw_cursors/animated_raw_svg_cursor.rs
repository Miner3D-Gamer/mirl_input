use super::super::*;


/// A superset of [`RawSVGCursor`] for animation support
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnimatedRawSVGCursor {
    /// The frames of the animation
    pub cursors: Vec<RawSVGCursor>,
}
impl AnimatedRawSVGCursor {
    /// Create a new [`AnimatedRawSVGCursor`]
    #[must_use]
    pub const fn new_from_list(list: Vec<RawSVGCursor>) -> Self {
        Self {
            cursors: list,
        }
    }
}