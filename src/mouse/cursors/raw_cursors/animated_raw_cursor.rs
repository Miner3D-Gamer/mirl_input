use mirl_buffer::prelude::*;

use super::super::*;

/// A trait noting which values are allowed to be used inside of [`AnimatedRawCursor`]
pub trait AllowedInAnimatedRawCursor {}
impl AllowedInAnimatedRawCursor for RawCursor {}
impl AllowedInAnimatedRawCursor for &RawCursor {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// A superset of [`RawCursor`] with animation support
///
/// All cursors have the same size and hotspot
pub struct AnimatedRawCursor<Cursor: AllowedInAnimatedRawCursor = RawCursor> {
    /// All [`RawCursor`] are the same size and a valid size
    ///
    /// Not recommended to access manually
    pub cursors: Vec<(Cursor, u32)>,
}

impl<T: AllowedInAnimatedRawCursor> AnimatedRawCursor<T> {
    #[must_use]
    /// Get the frame at the given index, returns [`None`] when the index is greater than the frame count
    pub fn get_frame(&self, index: usize) -> Option<&T> {
        self.cursors.get(index).map(|x| &x.0)
    }
    #[must_use]
    /// Get the frame at the given index without checking
    ///
    /// # Safety
    /// You have to make sure the idx is valid
    pub unsafe fn get_frame_unchecked(&self, index: usize) -> &T {
        &unsafe { self.cursors.get_unchecked(index) }.0
    }

    #[must_use]
    /// Get the amount of cursors
    pub const fn len(&self) -> usize {
        self.cursors.len()
    }
    #[must_use]
    /// Check if
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl AnimatedRawCursor<&RawCursor> {
    #[must_use]
    /// Clone the underlying [`RawCursor`] references to gain ownership
    pub fn to_owned(self) -> AnimatedRawCursor<RawCursor> {
        AnimatedRawCursor {
            cursors: self.cursors.iter().map(|x| ((*x.0).clone(), x.1)).collect(),
        }
    }
}
impl AnimatedRawCursor {
    /// Create an animated cursor from a list of raw cursors as well as
    ///
    /// # Errors
    /// [`AnimatedRawCursorError`]
    pub fn from_list(list: Vec<(RawCursor, u32)>) -> Result<Self, AnimatedRawCursorError> {
        if list.is_empty() {
            return Err(AnimatedRawCursorError::EmptyList);
        }

        let metrics = unsafe { list.get_unchecked(0) }.0.image.get_size();
        let mut wrong = None;
        for (idx, c) in list.iter().enumerate().skip(1) {
            if c.0.image.get_size() != metrics {
                wrong = Some(idx);
                break;
            }
        }
        if let Some(idx) = wrong {
            return Err(AnimatedRawCursorError::InvalidSize(list, idx));
        }
        Ok(Self { cursors: list })
    }
    #[must_use]
    /// Get the hotspot of this [`AnimatedRawCursor`]
    pub fn get_hotspot(&self) -> (u8, u8) {
        unsafe { self.cursors.get_unchecked(0).0.hotspot }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The errors that could happen when trying to create an [`AnimatedRawCursor`]
pub enum AnimatedRawCursorError {
    /// The [`Vec<AnimatedRawCursor>`] is empty
    EmptyList,
    /// The list that was inputted with the first item that has the wrong size
    InvalidSize(Vec<(RawCursor, u32)>, usize),
}
