// use super::super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// Raw SVG cursor data
///
/// # Safety
/// When creating this struct, you yourself are required to make sure that the SVG data is valid
pub struct RawSVGCursor {
    /// The SVG data (Optionally animated)
    pub data: String,
    /// The hotspot (where the click happens)
    ///
    /// This is relative to the 255x255 resolution
    pub hotspot: (u8, u8),
}
impl core::default::Default for RawSVGCursor {
    /// # Safety
    /// This function creates [`RawSVGCursor`] in an illegal state.
    /// You must manually set it afterwards.
    fn default() -> Self {
        Self {
            data: String::default(),
            hotspot: core::default::Default::default(),
        }
    }
}
