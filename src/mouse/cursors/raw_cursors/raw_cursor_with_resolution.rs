use mirl_buffer::prelude::*;

use super::super::*;
use crate::mouse::RawCursorFrameContainer;

impl RawCursorFrameContainer for RawCursorWithResolution<&RawCursor> {
    fn get_all_cursors_ref(&self) -> Vec<&RawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec().unwrap_unchecked() }
    }
    fn get_hotspot(&self) -> (u8, u8) {
        self.get_hotspot()
    }
}

impl<'a> From<&'a RawCursorWithResolution<RawCursor>> for RawCursorWithResolution<&'a RawCursor> {
    fn from(value: &'a RawCursorWithResolution<RawCursor>) -> Self {
        RawCursorWithResolution {
            x16: value.x16.as_ref(),
            x32: value.x32.as_ref(),
            x48: value.x48.as_ref(),
            x64: value.x64.as_ref(),
            x96: value.x96.as_ref(),
            x128: value.x128.as_ref(),
            x256: value.x256.as_ref(),
        }
    }
}
/// A raw cursors with multiple possible resolutions
///
/// Must contain at least 1 [`RawCursor`]
pub type RawCursorWithResolution<Cursor: AllowedInRawCursorWithResolution = RawCursor> =
    CustomCursorResolution<Option<Cursor>>;

impl RawCursorWithResolution<&RawCursor> {
    #[must_use]
    /// Get the resolution of the stored cursors
    pub fn get_hotspot(&self) -> (u8, u8) {
        unsafe { self.to_non_empty_vec_ref().unwrap_unchecked()[0].hotspot }
    }

    #[must_use]
    /// Get the cursor with the lowest resolution available
    ///
    /// ---
    ///
    /// For the cursor with the highest use [`get_highest_resolution`](Self::get_highest_resolution)
    ///
    /// To get all available cursors use [`get_all_cursors_ref`](Self::get_all_cursors_ref)
    pub const fn get_lowest_resolution_cursor(&self) -> &RawCursor {
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
    pub const fn get_highest_resolution_cursor(&self) -> &RawCursor {
        // Safety: One cursor must always exist
        unsafe { self.get_highest_resolution().unwrap_unchecked() }
    }

    #[must_use]
    /// Get a ref to all cursors from lowest to highest resolution
    ///
    /// TODO: Use `NonEmptyVec` for the return
    pub fn get_all_cursors_ref(&self) -> Vec<&RawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec().unwrap_unchecked() }
    }
}

impl RawCursorWithResolution {
    #[must_use]
    /// Get the resolution of the stored cursors
    pub fn get_hotspot(&self) -> (u8, u8) {
        unsafe { self.to_non_empty_vec_ref().unwrap_unchecked()[0].hotspot }
    }

    #[must_use]
    /// Get the cursor with the lowest resolution available
    ///
    /// ---
    ///
    /// For the cursor with the highest use [`get_highest_resolution`](Self::get_highest_resolution)
    ///
    /// To get all available cursors use [`get_all_cursors_ref`](Self::get_all_cursors_ref)
    pub const fn get_lowest_resolution_cursor(&self) -> &RawCursor {
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
    pub const fn get_highest_resolution_cursor(&self) -> &RawCursor {
        // Safety: One cursor must always exist
        unsafe { self.get_highest_resolution().unwrap_unchecked() }
    }
    #[must_use]
    /// Get all cursors from lowest to highest resolution
    ///
    /// TODO: Use `NonEmptyVec` for the return
    pub fn get_all_cursors(self) -> Vec<RawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec().unwrap_unchecked() }
    }
    #[must_use]
    /// Get a ref to all cursors from lowest to highest resolution
    ///
    /// TODO: Use `NonEmptyVec` for the return
    pub fn get_all_cursors_ref(&self) -> Vec<&RawCursor> {
        // Safety: One cursor must always exist
        unsafe { self.to_non_empty_vec_ref().unwrap_unchecked() }
    }

    /// Create a new resolution from a list of raw cursors
    ///
    /// If a cursor with an invalid cursor resolution is given, it is ignored
    ///
    /// # Errors
    /// When a [`RawCursor`] has an invalid size
    /// When not all cursors have the same same hotspot
    ///
    ///
    /// TODO: Use `NonEmptyVec` from `mirl_collections` to eliminate the check against default at the end
    pub fn new_from_list(list: Vec<RawCursor>) -> Result<Self, (Vec<RawCursor>, usize)> {
        let mut n = Self::default();
        let mut list = list;
        for idx in 0..list.len() {
            let item = unsafe { list.pop().unwrap_unchecked() };
            if let Some(bad) = n.insert_raw_cursor(item) {
                let mut already_inserted = n.to_non_empty_vec().unwrap_or_default();
                already_inserted.push(bad);
                already_inserted.extend(list);
                return Err((already_inserted, idx));
            }
        }
        if n == Self::default() {
            Err((Vec::new(), usize::MAX))
        } else {
            Ok(n)
        }
    }
    #[must_use]
    // #[allow(must_use)]
    /// Insert a raw cursor into [`Self`](RawCursorWithResolution), if the cursors has the wrong resolution it returns the cursor
    pub fn insert_raw_cursor(&mut self, cursor: RawCursor) -> Option<RawCursor> {
        if let Some(res) = CursorResolution::from_sized_resolution(cursor.image.get_size()) {
            unsafe {
                self.insert_raw_cursor_with_resolution(cursor, res);
            }
            None
        } else {
            Some(cursor)
        }
    }
    /// Insert the cursor if in the given resolution
    ///
    /// # Safety
    /// You must check if the cursor actually has the said resolution
    pub unsafe fn insert_raw_cursor_with_resolution(
        &mut self,
        cursor: RawCursor,
        res: CursorResolution,
    ) {
        match res {
            CursorResolution::X16 => self.x16 = Some(cursor),
            CursorResolution::X32 => self.x32 = Some(cursor),
            CursorResolution::X48 => self.x48 = Some(cursor),
            CursorResolution::X64 => self.x64 = Some(cursor),
            CursorResolution::X96 => self.x96 = Some(cursor),
            CursorResolution::X128 => self.x128 = Some(cursor),
            CursorResolution::X256 => self.x256 = Some(cursor),
        }
    }
}

/// What types are allowed to be used in [`RawCursorWithResolution`]
pub trait AllowedInRawCursorWithResolution {}
impl AllowedInRawCursorWithResolution for RawCursor {}
impl AllowedInRawCursorWithResolution for &RawCursor {}
