/// Cursor related stuff
pub mod cursors;

/// Load cursors dynamically
#[cfg(feature = "cursor_loading")]
pub mod loading;

mod button_state;
pub use button_state::*;
mod mouse_button_state;
pub use mouse_button_state::*;
mod mouse_snapshot;
pub use mouse_snapshot::*;

use crate::mouse::cursors::{
    CursorResolution, CustomCursorResolution, DefaultCursors, RawCursor,
    RawSVGCursor,
};

/// A struct holding the state of a mouse
pub mod mouse_manager;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Things that could go wrong while loading a cursor
pub enum LoadCursorError {
    /// Image data corrupt
    ///
    /// TODO: Make this it's own enum instead of a string
    InvalidImageData(String),
    /// Unknown
    Misc(String),
    /// A tempfile could not be created
    UnableToCreateTempfile,
    /// A tempfile could not be removed
    UnableToDeleteTempfile,
    /// Os could not load the cursor even though the cursor data has been constructed
    OsError,
    /// When the hotspot is in an invalid position
    InvalidHotspot,
    /// The current OS is not supported
    UnsupportedOs,
}
/// In what resolutions the custom cursor should be loaded
///
/// Why would you want multiple resolutions of a single cursor?
///
/// Windows scales cursors weirdly, so having multiple native sizes makes it looks nicer
pub type DefaultCustomCursorResolutionSettings = CustomCursorResolution<bool>;

impl core::default::Default for DefaultCustomCursorResolutionSettings {
    fn default() -> Self {
        Self {
            x16: false,
            x32: true,
            x48: false,
            x64: true,
            x96: false,
            x128: false,
            x256: false,
        }
    }
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// Supported (and unsupported) mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum MouseButton {
    #[default]
    /// ✨ The left mouse button ✨
    Left,
    /// ✨ The right mouse button ✨
    Right,
    /// ✨ The button between the left and right mouse buttons ✨
    Middle,
    /// An extra niche button some mice have
    Extra1,
    /// Another extra niche button some mice have
    Extra2,
    /// A freakish amalgamation of human invention
    Extra3,
    /// No one should be allowed this much power.
    Extra4,
    /// You can't expect to be able to expect everything ¯\_(ツ)_/¯
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// The shape of the default cursors has been defined but their color hasn't
pub struct DefaultCursorColorInfo {
    /// The main colour used in the cursor
    pub primary_color: u32,
    /// An accent colour, often a darker/lighter shade of the primary color
    pub secondary_color: u32,
}

/// Used by `create_cur` to obtain the raw cursors from any container struct
pub trait RawCursorFrameContainer {
    /// Get all raw frames cursor frame by ref
    fn get_all_cursors_ref(&self) -> Vec<&RawCursor>;
    /// Get the hotspot of the held cursors
    fn get_hotspot(&self) -> (u8, u8);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// Things that could go wrong when loading the default cursors
///
/// ---
///
/// These errors are not possible in practice and only exists for testing purposes in case we messed up.
///
/// You are safe using [`unwrap_unchecked()`](core::result::Result::unwrap_unchecked) on this error.
/// (In the case where UB happens, which there should never, [`mirl_windowing`](crate) takes responsibility)
pub enum DefaultCursorLoadError {
    /// There was a problem with the decompressing of the cursor bytes
    ///
    /// Sadly no other info can be provided since their error enum doesn't support the default derives
    SevenZ,
    /// Virtual file cannot be found.
    InfoNotFound,
    /// A Base cursor file was not defined
    CursorNotDefined,
    /// There was a problem when rasterizing the svg.
    ///
    /// Sadly no other info can be provided since their error enum doesn't support the default derives
    RasterizeSVG,

    /// Unknown error
    Unknown(&'static str),
}
#[cfg(feature = "cursor_loading")]
impl From<sevenz_rust2::Error> for DefaultCursorLoadError {
    fn from(_value: sevenz_rust2::Error) -> Self {
        Self::SevenZ
    }
}

impl From<Option<RawSVGCursor>> for DefaultCursorLoadError {
    fn from(_value: Option<RawSVGCursor>) -> Self {
        Self::CursorNotDefined
    }
}
// impl From<Option<&(&str, (u8, u8), DefaultCursorsSelection)>>
//     for DefaultCursorLoadError
// {
//     fn from(
//         _value: Option<&(&str, (u8, u8), DefaultCursorsSelection)>,
//     ) -> Self {
//         Self::InfoNotFound
//     }
// }

/// Return all default cursors
pub const trait LoadDefaultCustomCursor: core::marker::Sized {
    /// Get all default custom cursors in the current cursor type
    ///
    /// # Errors
    /// [`LoadCursorError`]
    fn get_default_custom_cursors(
        size: CursorResolution,
        color_info: DefaultCursorColorInfo,
    ) -> Result<DefaultCursors<Self>, LoadCursorError>;
}
