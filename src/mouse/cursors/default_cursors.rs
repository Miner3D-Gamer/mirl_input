use super::*;

/// Default cursors `mirl_windowing` provides
///
/// The true default is named "Pointer"
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct DefaultCursors<Cursor> {
    /// Default Pointer
    pub alias: Cursor,
    /// Resize vertically + Resize horizontally
    pub resize_all: Cursor,
    /// Arrow pointing to the bottom left ⬋
    pub arrow_bottom_left: Cursor,
    /// Arrow pointing to the bottom right ⬊
    pub arrow_bottom_right: Cursor,
    /// Arrow down with a _ at the end
    pub arrow_bottom_stop: Cursor,
    /// A plus shape
    pub cell: Cursor,
    /// Default cursor rotated to be vertical
    pub centered_pointer: Cursor,
    // /// Horizontal resizing
    // pub resize_horizontal: Cursor,
    /// Eyedropper
    pub color_picker: Cursor,
    /// Default cursor with ≡ attached
    pub context_menu: Cursor,
    /// Default cursor with a plus
    pub copy: Cursor,
    /// Cross
    pub crosshair: Cursor,
    /// Default Pointer
    pub pointer: Cursor,
    /// Closed hand
    pub closed_hand: Cursor,
    /// Closed hand with an 🚫 attached
    pub closed_hand_no_drop: Cursor,
    /// Arrow pointing down
    pub arrow_down: Cursor,
    /// Tip of an ink pen
    pub draft: Cursor,
    /// Small pointers in all directions like this: ◄ ►
    pub fleur: Cursor,
    /// Question mark
    pub help: Cursor,
    /// Arrow pointing left
    pub arrow_left: Cursor,
    /// Arrow left with a stopper |←
    pub arrow_left_stop: Cursor,
    /// Default cursor with a 🚫 attached
    pub no_drop: Cursor,
    /// "🚫"
    pub not_allowed: Cursor,
    /// Open hand
    pub open_hand: Cursor,
    /// A Pencil
    pub pencil: Cursor,
    /// Skull
    pub pirate: Cursor,
    /// Hand with pointing index finger
    pub hand: Cursor,
    /// Arrow pointing right
    pub arrow_right: Cursor,
    /// Mirrored version of normal cursor
    pub mirrored_pointer: Cursor,
    /// Arrow pointing right with a stopper →|
    pub arrow_right_stop: Cursor,
    /// Resize top right to bottom left
    pub resize_nesw: Cursor,
    /// Resize top left to bottom right
    pub resize_nwse: Cursor,
    /// Resize horizontally
    pub resize_horizontal: Cursor,
    /// Resize vertically
    pub resize_vertical: Cursor,
    /// I Beam
    pub text: Cursor,
    /// Arrow pointing up top left
    pub arrow_top_left: Cursor,
    /// Arrow pointing up top right
    pub arrow_top_right: Cursor,
    /// Arrow pointing up with an _ on top
    pub arrow_top_stop: Cursor,
    /// Arrow pointing up
    pub arrow_up: Cursor,
    /// I Beam rotated 90°
    pub vertical_text: Cursor,
    /// Magnifying glass with plus
    pub zoom_in: Cursor,
    /// Magnifying glass with minus
    pub zoom_out: Cursor,
}
impl<Cursor> DefaultCursors<Cursor> {
    /// Get the selected cursor as a ref
    pub const fn get_cursor(&self, cursor: DefaultCursorsSelection) -> &Cursor {
        match cursor {
            DefaultCursorsSelection::Alias => &self.alias,
            DefaultCursorsSelection::ResizeAll => &self.resize_all,
            DefaultCursorsSelection::ArrowBottomLeft => &self.arrow_bottom_left,
            DefaultCursorsSelection::ArrowBottomRight => {
                &self.arrow_bottom_right
            }
            DefaultCursorsSelection::ArrowBottomStop => &self.arrow_bottom_stop,
            DefaultCursorsSelection::Cell => &self.cell,
            DefaultCursorsSelection::CenteredPointer => &self.centered_pointer,
            DefaultCursorsSelection::ColorPicker => &self.color_picker,
            DefaultCursorsSelection::ContextMenu => &self.context_menu,
            DefaultCursorsSelection::Copy => &self.copy,
            DefaultCursorsSelection::Crosshair => &self.crosshair,
            DefaultCursorsSelection::Pointer => &self.pointer,
            DefaultCursorsSelection::ClosedHand => &self.closed_hand,
            DefaultCursorsSelection::ClosedHandNoDrop => {
                &self.closed_hand_no_drop
            }
            DefaultCursorsSelection::ArrowDown => &self.arrow_down,
            DefaultCursorsSelection::Draft => &self.draft,
            DefaultCursorsSelection::Fleur => &self.fleur,
            DefaultCursorsSelection::Help => &self.help,
            DefaultCursorsSelection::ArrowLeft => &self.arrow_left,
            DefaultCursorsSelection::ArrowLeftStop => &self.arrow_left_stop,
            DefaultCursorsSelection::NoDrop => &self.no_drop,
            DefaultCursorsSelection::NotAllowed => &self.not_allowed,
            DefaultCursorsSelection::OpenHand => &self.open_hand,
            DefaultCursorsSelection::Pencil => &self.pencil,
            DefaultCursorsSelection::Pirate => &self.pirate,
            DefaultCursorsSelection::Hand => &self.hand,
            DefaultCursorsSelection::ArrowRight => &self.arrow_right,
            DefaultCursorsSelection::MirroredPointer => &self.mirrored_pointer,
            DefaultCursorsSelection::ArrowRightStop => &self.arrow_right_stop,
            DefaultCursorsSelection::ResizeNWSE => &self.resize_nwse,
            DefaultCursorsSelection::ResizeNESW => &self.resize_nesw,
            DefaultCursorsSelection::ResizeVertical => &self.resize_vertical,
            DefaultCursorsSelection::ResizeHorizontal => {
                &self.resize_horizontal
            }
            DefaultCursorsSelection::Text => &self.text,
            DefaultCursorsSelection::ArrowTopLeft => &self.arrow_top_left,
            DefaultCursorsSelection::ArrowTopRight => &self.arrow_top_right,
            DefaultCursorsSelection::ArrowTopStop => &self.arrow_top_stop,
            DefaultCursorsSelection::ArrowUp => &self.arrow_up,
            DefaultCursorsSelection::VerticalText => &self.vertical_text,
            DefaultCursorsSelection::ZoomIn => &self.zoom_in,
            DefaultCursorsSelection::ZoomOut => &self.zoom_out,
        }
    }
    /// Get the selected cursor as a mutable red
    pub const fn get_cursor_mut(
        &mut self,
        cursor: DefaultCursorsSelection,
    ) -> &mut Cursor {
        match cursor {
            DefaultCursorsSelection::Alias => &mut self.alias,
            DefaultCursorsSelection::ResizeAll => &mut self.resize_all,
            DefaultCursorsSelection::ArrowBottomLeft => {
                &mut self.arrow_bottom_left
            }
            DefaultCursorsSelection::ArrowBottomRight => {
                &mut self.arrow_bottom_right
            }
            DefaultCursorsSelection::ArrowBottomStop => {
                &mut self.arrow_bottom_stop
            }
            DefaultCursorsSelection::Cell => &mut self.cell,
            DefaultCursorsSelection::CenteredPointer => {
                &mut self.centered_pointer
            }
            DefaultCursorsSelection::ColorPicker => &mut self.color_picker,
            DefaultCursorsSelection::ContextMenu => &mut self.context_menu,
            DefaultCursorsSelection::Copy => &mut self.copy,
            DefaultCursorsSelection::Crosshair => &mut self.crosshair,
            DefaultCursorsSelection::Pointer => &mut self.pointer,
            DefaultCursorsSelection::ClosedHand => &mut self.closed_hand,
            DefaultCursorsSelection::ClosedHandNoDrop => {
                &mut self.closed_hand_no_drop
            }
            DefaultCursorsSelection::ArrowDown => &mut self.arrow_down,
            DefaultCursorsSelection::Draft => &mut self.draft,
            DefaultCursorsSelection::Fleur => &mut self.fleur,
            DefaultCursorsSelection::Help => &mut self.help,
            DefaultCursorsSelection::ArrowLeft => &mut self.arrow_left,
            DefaultCursorsSelection::ArrowLeftStop => &mut self.arrow_left_stop,
            DefaultCursorsSelection::NoDrop => &mut self.no_drop,
            DefaultCursorsSelection::NotAllowed => &mut self.not_allowed,
            DefaultCursorsSelection::OpenHand => &mut self.open_hand,
            DefaultCursorsSelection::Pencil => &mut self.pencil,
            DefaultCursorsSelection::Pirate => &mut self.pirate,
            DefaultCursorsSelection::Hand => &mut self.hand,
            DefaultCursorsSelection::ArrowRight => &mut self.arrow_right,
            DefaultCursorsSelection::MirroredPointer => {
                &mut self.mirrored_pointer
            }
            DefaultCursorsSelection::ArrowRightStop => {
                &mut self.arrow_right_stop
            }
            DefaultCursorsSelection::ResizeNWSE => &mut self.resize_nwse,
            DefaultCursorsSelection::ResizeNESW => &mut self.resize_nesw,
            DefaultCursorsSelection::ResizeVertical => {
                &mut self.resize_vertical
            }
            DefaultCursorsSelection::ResizeHorizontal => {
                &mut self.resize_horizontal
            }
            DefaultCursorsSelection::Text => &mut self.text,
            DefaultCursorsSelection::ArrowTopLeft => &mut self.arrow_top_left,
            DefaultCursorsSelection::ArrowTopRight => &mut self.arrow_top_right,
            DefaultCursorsSelection::ArrowTopStop => &mut self.arrow_top_stop,
            DefaultCursorsSelection::ArrowUp => &mut self.arrow_up,
            DefaultCursorsSelection::VerticalText => &mut self.vertical_text,
            DefaultCursorsSelection::ZoomIn => &mut self.zoom_in,
            DefaultCursorsSelection::ZoomOut => &mut self.zoom_out,
        }
    }
}
