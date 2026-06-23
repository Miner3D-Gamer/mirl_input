use mirl_buffer::prelude::*;

use super::super::*;
use crate::mouse::{DefaultCursorColorInfo, DefaultCursorLoadError};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
/// A raw representation of a cursor
///
/// The hotspot will never be outside the image bounds and the image will always have a legal size
///
/// Manually setting the hotspot/image will shift all safety liabilities to you
pub struct RawCursor {
    /// The visuals of the cursor
    pub image: Buffer,
    /// The spot where the cursor clicks (shifts image if not at Pos[0, 0])
    pub hotspot: (u8, u8),
}
impl RawCursor {
    #[must_use]
    /// Get the hotspot
    pub const fn get_hotspot(&self) -> (u8, u8) {
        self.hotspot
    }

    #[must_use]
    /// Create a new raw cursor, returning [`None`] if the hotspot is outside the image bounds or the image has an invalid resolution
    pub fn new(image: Buffer, hotspot: (u8, u8)) -> Option<Self> {
        CursorResolution::from_sized_resolution(image.get_size())?;
        if hotspot.0 as usize >= image.width
            || hotspot.1 as usize >= image.height
        {
            None
        } else {
            Some(Self {
                image,
                hotspot,
            })
        }
    }
}

impl RawCursor {
    #[must_use]
    /// Rasterize the raw SVG into a [`Buffer`] image we can use more normally
    ///
    /// # Safety
    /// If you do not verify the integrity of the insides, this function will cause UB
    pub unsafe fn from_raw_svg_cursor_unchecked(
        raw_cursor: RawSVGCursor,
        size: CursorResolution,
        color_info: DefaultCursorColorInfo,
    ) -> Self {
        unsafe {
            Self::from_raw_svg_cursor(raw_cursor, size, color_info)
                .unwrap_unchecked()
        }
    }
    /// Rasterize the raw SVG into a [`Buffer`] image we can use more normally
    ///
    /// # Errors
    /// [`DefaultCursorLoadError`]
    pub fn from_raw_svg_cursor(
        raw_cursor: RawSVGCursor,
        size: CursorResolution,
        color_info: DefaultCursorColorInfo,
    ) -> Result<Self, DefaultCursorLoadError> {
        let wanted_size: u32 = u32::from(size.get_size());
        let data = raw_cursor.data;
        // TODO: Is this correct?
        // TODO: The cursors were maybe designed for for 24x24 instead of 16x16. We may need to scretch the hotspot.
        let data = data.replace(
            "{primary}",
            &mirl_graphics::misc::color_to_hex(color_info.primary_color),
        );
        let data = data.replace(
            "{secondary}",
            &mirl_graphics::misc::color_to_hex(color_info.secondary_color),
        );

        let image_data =
            mirl_graphics::misc::rasterize_svg(&data, wanted_size, wanted_size)
                .map_err(|_| DefaultCursorLoadError::RasterizeSVG)?;
        let buffer = mirl_graphics::misc::pixmap_to_buffer(&image_data);
        Ok(Self {
            image: buffer,
            hotspot: raw_cursor.hotspot,
        })
    }
}
