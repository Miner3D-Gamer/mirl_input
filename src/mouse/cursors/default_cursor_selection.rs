use crate::mouse::DefaultCursorLoadError;

// use super::*;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(wincode = false))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
/// A direct reference to the default cursors
pub enum DefaultCursorsSelection {
    /// Default cursor with an extra arrow (e.g. clickable text)
    /// [`DefaultCursors::alias`]
    Alias,

    /// Resize vertically + Resize horizontally
    /// [`DefaultCursors::resize_all`]
    ResizeAll,

    /// Arrow pointing to the bottom left ⬋
    /// [`DefaultCursors::arrow_bottom_left`]
    ArrowBottomLeft,

    /// Arrow pointing to the bottom right ⬊
    /// [`DefaultCursors::arrow_bottom_right`]
    ArrowBottomRight,

    /// Arrow down with a _ at the end
    /// [`DefaultCursors::arrow_bottom_stop`]
    ArrowBottomStop,

    /// A plus shape
    /// [`DefaultCursors::cell`]
    Cell,

    /// Default cursor rotated to be vertical
    /// [`DefaultCursors::centered_pointer`]
    CenteredPointer,

    /// Eyedropper
    /// [`DefaultCursors::color_picker`]
    ColorPicker,

    /// Default cursor with ≡ attached
    /// [`DefaultCursors::context_menu`]
    ContextMenu,

    /// Default cursor with a plus
    /// [`DefaultCursors::copy`]
    Copy,

    /// Cross
    /// [`DefaultCursors::crosshair`]
    Crosshair,

    #[default]
    /// Hand with pointing index finger
    /// [`DefaultCursors::pointer`]
    Pointer,

    /// Closed hand
    /// [`DefaultCursors::closed_hand`]
    ClosedHand,

    /// Closed hand with an 🚫 attached
    /// [`DefaultCursors::closed_hand_no_drop`]
    ClosedHandNoDrop,

    /// Arrow pointing down
    /// [`DefaultCursors::arrow_down`]
    ArrowDown,

    /// Tip of an ink pen
    /// [`DefaultCursors::draft`]
    Draft,

    /// Small pointers in all directions like this: ◄ ►
    /// [`DefaultCursors::fleur`]
    Fleur,

    /// Question mark
    /// [`DefaultCursors::help`]
    Help,

    /// Arrow pointing left
    /// [`DefaultCursors::arrow_left`]
    ArrowLeft,

    /// Arrow left with a stopper |←
    /// [`DefaultCursors::arrow_left_stop`]
    ArrowLeftStop,

    /// Default cursor with a 🚫 attached
    /// [`DefaultCursors::no_drop`]
    NoDrop,

    /// "🚫"
    /// [`DefaultCursors::not_allowed`]
    NotAllowed,

    /// Open hand
    /// [`DefaultCursors::open_hand`]
    OpenHand,

    /// A Pencil
    /// [`DefaultCursors::pencil`]
    Pencil,

    /// Skull
    /// [`DefaultCursors::pirate`]
    Pirate,

    /// Hand with pointing index finger
    /// [`DefaultCursors::hand`]
    Hand,

    /// Arrow pointing right
    /// [`DefaultCursors::arrow_right`]
    ArrowRight,

    /// Mirrored version of normal cursor
    /// [`DefaultCursors::mirrored_pointer`]
    MirroredPointer,

    /// Arrow pointing right with a stopper →|
    /// [`DefaultCursors::arrow_right_stop`]
    ArrowRightStop,

    /// Resize top left to bottom right
    /// [`DefaultCursors::resize_nwse`]
    ResizeNWSE,

    /// Resize top right to bottom left
    /// [`DefaultCursors::resize_nesw`]
    ResizeNESW,

    /// Resize vertically
    /// [`DefaultCursors::resize_vertical`]
    ResizeVertical,

    /// Horizontal resizing
    /// [`DefaultCursors::resize_horizontal`]
    ResizeHorizontal,

    /// I Beam
    /// [`DefaultCursors::text`]
    Text,

    /// Arrow pointing up top left
    /// [`DefaultCursors::arrow_top_left`]
    ArrowTopLeft,

    /// Arrow pointing up top right
    /// [`DefaultCursors::arrow_top_right`]
    ArrowTopRight,

    /// Arrow pointing up with an _ on top
    /// [`DefaultCursors::arrow_top_stop`]
    ArrowTopStop,

    /// Arrow pointing up
    /// [`DefaultCursors::arrow_up`]
    ArrowUp,

    /// I Beam rotated 90°
    /// [`DefaultCursors::vertical_text`]
    VerticalText,

    /// Magnifying glass with plus
    /// [`DefaultCursors::zoom_in`]
    ZoomIn,

    /// Magnifying glass with minus
    /// [`DefaultCursors::zoom_out`]
    ZoomOut,
}

impl From<Option<&(&str, (u8, u8), DefaultCursorsSelection)>>
    for DefaultCursorLoadError
{
    fn from(
        _value: Option<&(&str, (u8, u8), DefaultCursorsSelection)>,
    ) -> Self {
        Self::InfoNotFound
    }
}
