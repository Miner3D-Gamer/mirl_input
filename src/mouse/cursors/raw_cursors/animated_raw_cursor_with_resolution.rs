use super::super::*;

/// A raw cursors with multiple possible resolutions
///
/// Must contain at least 1 [`AnimatedRawCursor`]
pub type AnimatedRawCursorWithResolution =
    CustomCursorResolution<Option<AnimatedRawCursor>>;

impl AnimatedRawCursorWithResolution {
    #[must_use]
    /// Get a list of frames
    pub fn to_raw_cursor_with_resolution(
        &self,
    ) -> Vec<RawCursorWithResolution<&RawCursor>> {
        fn get_frame(
            frame: Option<&AnimatedRawCursor>,
            idx: usize,
        ) -> Option<&RawCursor> {
            frame.as_ref().map(|x| unsafe { x.get_frame_unchecked(idx) })
        }
        let mut list = Vec::new();

        for frame in 0..self.get_lowest_resolution_cursor().cursors.len() {
            list.push(RawCursorWithResolution {
                x16: get_frame(self.x16.as_ref(), frame),
                x32: get_frame(self.x32.as_ref(), frame),
                x48: get_frame(self.x48.as_ref(), frame),
                x64: get_frame(self.x64.as_ref(), frame),
                x96: get_frame(self.x96.as_ref(), frame),
                x128: get_frame(self.x128.as_ref(), frame),
                x256: get_frame(self.x256.as_ref(), frame),
            });
        }

        list
    }
    #[must_use]
    /// Get the resolution of the stored cursors
    pub fn get_hotspot(&self) -> (u8, u8) {
        unsafe {
            self.to_non_empty_vec_ref()
                .unwrap_unchecked()
                .get_unchecked(0)
                .get_hotspot()
        }
    }

    #[must_use]
    /// Get the cursor with the lowest resolution available
    ///
    /// ---
    ///
    /// For the cursor with the highest use [`get_highest_resolution`](Self::get_highest_resolution)
    ///
    /// To get all available cursors use [`get_all_cursors_ref`](Self::get_all_cursors_ref)
    pub const fn get_lowest_resolution_cursor(&self) -> &AnimatedRawCursor {
        // Safety: One cursor must always exist
        unsafe { self.get_lowest_resolution().unwrap_unchecked() }
    }
    #[must_use]
    /// Get the cursor with the highest resolution available
    ///
    /// ---
    ///
    /// For the cursor with the highest use [`get_lowest_resolution`](Self::get_lowest_resolution_cursor)
    ///
    /// To get all available cursors use [`get_all_cursors_ref`](Self::get_all_cursors_ref)
    pub const fn get_highest_resolution_cursor(&self) -> &AnimatedRawCursor {
        // Safety: One cursor must always exist
        unsafe { self.get_highest_resolution().unwrap_unchecked() }
    }
    #[must_use]
    /// Get all cursors from lowest to highest resolution
    ///
    /// TODO: Use `NonEmptyVec` for the return
    pub fn get_all_cursors(self) -> Vec<AnimatedRawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec().unwrap_unchecked() }
    }
    #[must_use]
    /// Get a ref to all cursors from lowest to highest resolution
    ///
    /// TODO: Use `NonEmptyVec` for the return
    pub fn get_all_cursors_ref(&self) -> Vec<&AnimatedRawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec_ref().unwrap_unchecked() }
    }
}
