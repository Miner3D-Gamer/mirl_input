// TODO: Add support for the animated cursors
// TODO: Add more color customization for the individual custom default cursors

// use crate::windowing::WindowError;

// /// Set the cursor of the current window
// ///
// /// # Errors
// /// See [`WindowError`]
// pub fn use_cursor(
//     cursor: &Cursor,
//     #[cfg(all(feature = "glfw", not(target_arch = "wasm32")))]
//     glfw_window: Option<&mut glfw::Window>,
//     #[cfg(not(all(feature = "glfw", not(target_arch = "wasm32"))))]
//     _glfw_window: core::option::Option<()>,
// ) -> Result<(), WindowError> {
//     #[cfg(not(target_arch = "wasm32"))]
//     #[cfg(feature = "glfw")]
//     if let Some(additional_info) = glfw_window {
//         #[allow(irrefutable_let_patterns)]
//         if let Cursor::Glfw(new_cursor) = cursor {
//             use crate::graphics::buffer_to_pixel_image;

//             let given = new_cursor;
//             let buffer = &given.0;
//             let pixel: glfw::PixelImage = buffer_to_pixel_image(buffer);
//             //println!("Cursor dimensions: {} x {}", given.1, given.2);

//             // Check if dimensions make sense
//             if given.1 == 0 || given.2 == 0 || given.1 > 1024 || given.2 > 1024
//             {
//                 //println!("Error: Invalid cursor dimensions");
//                 return Err(WindowError::IncorrectSize(
//                     (given.1, given.2).try_tuple_into().unwrap_or_default(),
//                 ));
//             }
//             let new = glfw::Cursor::create_from_pixels(pixel, given.1, given.2);

//             additional_info.set_cursor(Some(new));
//         } else {
//             use crate::platform::windowing::WindowError;

//             return Err(WindowError::Misc(format!(
//                 "Cursor of type Cursor::Glfw expected but got {cursor:?} instead"
//             )));
//             //panic!("Cannot set GLFW cursor -> No cursors provided");
//         }
//         #[allow(unreachable_code)]
//         return Ok(());
//     }
//     match cursor {
//         #[cfg(target_os = "windows")]
//         #[cfg(feature = "svg")]
//         #[cfg(feature = "system")]
//         Cursor::Win(cursor) => {
//             unsafe {
//                 // Update the passive cursor provider windows uses
//                 cursors_windows::update_cursor(cursor);
//                 // Update currently used cursor (Passive provider only gets checked when mouse moves)
//                 cursors_windows::set_cursor(cursor);
//             }
//         }

//         #[cfg(target_os = "linux")]
//         Cursor::X11(xcursor_id) => {
//             return Err(WindowError::NotImplemented);
//             // Use the X11 cursor ID
//             // panic!("X11 cursor id: {}", xcursor_id.unwrap());
//         }

//         #[cfg(target_os = "macos")]
//         Cursor::Mac(_ptr) => {
//             return Err(WindowError::NotImplemented);
//             // Use macOS cursor pointer (e.g., NSCursor*)
//             //panic!("macOS cursor pointer: {:?}", ptr.unwrap());
//         }
//         Cursor::Glfw(_) => {
//             return Err(WindowError::Misc("Impossible".to_string()));
//             //panic!("Cannot set GLFW cursor -> Not a GLFW context");
//         }
//     }
//     #[allow(unreachable_code)]
//     Ok(())
// }

use crate::mouse::{DefaultCursorColorInfo, cursors::*};

/// All default svg cursors are made with the size 24x24 in mind I believe.
///
/// Psychopathic.
pub const DEFAULT_CURSOR_SIZE: u8 = 24;

/// All cursor data compressed into a 7z
pub static CURSORS_7Z: &[u8] = include_bytes!("cursors.7z");
#[rustfmt::skip]
/// Info about the cursors stored in [`CURSORS_7Z`]
pub static DEFAULT_CURSOR_INFO: &[(&[&str], (u8, u8), DefaultCursorsSelection)] = &[
    (&["default"], (6, 4), DefaultCursorsSelection::Pointer),
    (&["alias"], (6, 4), DefaultCursorsSelection::Alias),
    (&["all_scroll"], (16, 16), DefaultCursorsSelection::ResizeAll),
    (&["bottom_left_corner"], (5, 27), DefaultCursorsSelection::ArrowBottomLeft),
    (&["bottom_right_corner"], (29, 27), DefaultCursorsSelection::ArrowBottomRight),
    (&["bottom_side"], (16, 28), DefaultCursorsSelection::ArrowBottomStop),
    (&["cell"], (16, 16), DefaultCursorsSelection::Cell),
    (&["center_ptr"], (15, 7), DefaultCursorsSelection::CenteredPointer),
    (&["color_picker"], (4, 29), DefaultCursorsSelection::ColorPicker),
    (&["context_menu"], (6, 4), DefaultCursorsSelection::ContextMenu),
    (&["copy"], (6, 4), DefaultCursorsSelection::Copy),
    (&["crosshair"], (16, 16), DefaultCursorsSelection::Crosshair),
    (&["closed_hand"], (16, 16), DefaultCursorsSelection::ClosedHand),
    (&["closed_hand_no_drop"], (16, 16), DefaultCursorsSelection::ClosedHandNoDrop),
    (&["down_arrow"], (16, 28), DefaultCursorsSelection::ArrowDown),
    (&["draft"], (4, 29), DefaultCursorsSelection::Draft),
    (&["fleur"], (16, 16), DefaultCursorsSelection::Fleur),
    (&["help"], (4, 4), DefaultCursorsSelection::Help),
    (&["left_arrow"], (4, 16), DefaultCursorsSelection::ArrowLeft),
    (&["left_side"], (4, 16), DefaultCursorsSelection::ArrowLeftStop),
    (&["no_drop"], (6, 4), DefaultCursorsSelection::NoDrop),
    (&["not_allowed"], (16, 16), DefaultCursorsSelection::NotAllowed),
    (&["open_hand"], (16, 16), DefaultCursorsSelection::OpenHand),
    (&["pencil"], (4, 29), DefaultCursorsSelection::Pencil),
    (&["pirate"], (16, 16), DefaultCursorsSelection::Pirate),
    (&["pointer"], (13, 7), DefaultCursorsSelection::Hand),
    (&["right_arrow"], (28, 16), DefaultCursorsSelection::ArrowRight),
    (&["right_ptr"], (28, 4), DefaultCursorsSelection::MirroredPointer),
    (&["right_side"], (28, 16), DefaultCursorsSelection::ArrowRightStop),
    (&["size_nesw"], (16, 16), DefaultCursorsSelection::ResizeNESW),
    (&["size_nwse"], (16, 16), DefaultCursorsSelection::ResizeNWSE),
    (&["size_hor"], (16, 16), DefaultCursorsSelection::ResizeHorizontal),
    (&["size_ver"], (16, 16), DefaultCursorsSelection::ResizeVertical),
    (&["text"], (15, 16), DefaultCursorsSelection::Text),
    (&["top_left_corner"], (9, 9), DefaultCursorsSelection::ArrowTopLeft),
    (&["top_right_corner"], (21, 9), DefaultCursorsSelection::ArrowTopRight),
    (&["top_side"], (16, 4), DefaultCursorsSelection::ArrowTopStop),
    (&["up_arrow"], (16, 4), DefaultCursorsSelection::ArrowUp),
    (&["vertical_text"], (16, 16), DefaultCursorsSelection::VerticalText),
    (&["zoom_in"], (16, 16), DefaultCursorsSelection::ZoomIn),
    (&["zoom_out"], (16, 16), DefaultCursorsSelection::ZoomOut),
];
#[allow(clippy::missing_panics_doc)] // This doesn't panic.
/// Get the default cursors in SVG form without any colors applied
///
/// # Errors
/// Any errors can be ignored. It is safe to use `.unwrap()` on this function
#[must_use]
pub fn get_raw_default_cursors_unconfigured() -> DefaultCursors<RawSVGCursor> {
    let reader = std::io::Cursor::new(CURSORS_7Z);

    let mut reader =
        sevenz_rust2::ArchiveReader::new(reader, sevenz_rust2::Password::empty()).unwrap();
    #[allow(clippy::type_complexity)]
    let cursor_infos: (
        Vec<Vec<String>>,
        Vec<(u8, u8)>,
        Vec<DefaultCursorsSelection>,
    ) = DEFAULT_CURSOR_INFO
        .iter()
        .map(|x| {
            (
                x.0.iter()
                    .map(|y| format!("{y}.svg"))
                    .collect::<Vec<String>>(),
                x.1,
                x.2,
            )
        })
        .collect();

    let mut default: DefaultCursors<RawSVGCursor> = DefaultCursors::default();

    // TODO: Reverse the lists and pop instead of indexing each time.
    for info_idx in 0..cursor_infos.0.len() {
        let files = unsafe { cursor_infos.0.get_unchecked(info_idx) };
        let hotspot = *unsafe { cursor_infos.1.get_unchecked(info_idx) };
        let id = *unsafe { cursor_infos.2.get_unchecked(info_idx) };
        println!("File names {}", files.join(", "));

        // let mut cursors = Vec::with_capacity(files.len());
        for file in files {
            let data = reader.read_file(file).unwrap();
            // Safety: All files are ascii only so utf8 compliant.
            let string = unsafe { String::from_utf8_unchecked(data) };
            // cursors.push(string);
            let cursor = RawSVGCursor {
                data: string,
                hotspot,
            };
            let c = default.get_cursor_mut(id);
            *c = cursor;
        }
    }

    default
}

#[allow(clippy::missing_panics_doc)] // This doesn't panic.
/// Get the default cursors in image + hotspot form ([`DefaultRawSVGCursor`])
///
/// # Errors
/// Any errors can be ignored. It is safe to use `.unwrap()` on this function
#[must_use]
pub fn get_raw_default_cursors(
    size: CursorResolution,
    color_info: DefaultCursorColorInfo,
) -> DefaultCursors<RawCursor> {
    let raw_cursors = get_raw_default_cursors_unconfigured();

    #[rustfmt::skip]
    let final_cursors = DefaultCursors {
        alias:                  RawCursor::from_raw_svg_cursor(raw_cursors.alias, size, color_info).unwrap(),
        resize_all:             RawCursor::from_raw_svg_cursor(raw_cursors.resize_all, size, color_info).unwrap(),
        arrow_bottom_left:      RawCursor::from_raw_svg_cursor(raw_cursors.arrow_bottom_left, size, color_info).unwrap(),
        arrow_bottom_right:     RawCursor::from_raw_svg_cursor(raw_cursors.arrow_bottom_right, size, color_info).unwrap(),
        arrow_bottom_stop:      RawCursor::from_raw_svg_cursor(raw_cursors.arrow_bottom_stop, size, color_info).unwrap(),
        cell:                   RawCursor::from_raw_svg_cursor(raw_cursors.cell, size, color_info).unwrap(),
        centered_pointer:       RawCursor::from_raw_svg_cursor(raw_cursors.centered_pointer, size, color_info).unwrap(),
        color_picker:           RawCursor::from_raw_svg_cursor(raw_cursors.color_picker, size, color_info).unwrap(),
        context_menu:           RawCursor::from_raw_svg_cursor(raw_cursors.context_menu, size, color_info).unwrap(),
        copy:                   RawCursor::from_raw_svg_cursor(raw_cursors.copy, size, color_info).unwrap(),
        crosshair:              RawCursor::from_raw_svg_cursor(raw_cursors.crosshair, size, color_info).unwrap(),
        pointer:                RawCursor::from_raw_svg_cursor(raw_cursors.pointer, size, color_info).unwrap(),
        closed_hand:            RawCursor::from_raw_svg_cursor(raw_cursors.closed_hand, size, color_info).unwrap(),
        closed_hand_no_drop:    RawCursor::from_raw_svg_cursor(raw_cursors.closed_hand_no_drop, size, color_info).unwrap(),
        arrow_down:             RawCursor::from_raw_svg_cursor(raw_cursors.arrow_down, size, color_info).unwrap(),
        draft:                  RawCursor::from_raw_svg_cursor(raw_cursors.draft, size, color_info).unwrap(),
        fleur:                  RawCursor::from_raw_svg_cursor(raw_cursors.fleur, size, color_info).unwrap(),
        help:                   RawCursor::from_raw_svg_cursor(raw_cursors.help, size, color_info).unwrap(),
        arrow_left:             RawCursor::from_raw_svg_cursor(raw_cursors.arrow_left, size, color_info).unwrap(),
        arrow_left_stop:        RawCursor::from_raw_svg_cursor(raw_cursors.arrow_left_stop, size, color_info).unwrap(),
        no_drop:                RawCursor::from_raw_svg_cursor(raw_cursors.no_drop, size, color_info).unwrap(),
        not_allowed:            RawCursor::from_raw_svg_cursor(raw_cursors.not_allowed, size, color_info).unwrap(),
        open_hand:              RawCursor::from_raw_svg_cursor(raw_cursors.open_hand, size, color_info).unwrap(),
        pencil:                 RawCursor::from_raw_svg_cursor(raw_cursors.pencil, size, color_info).unwrap(),
        pirate:                 RawCursor::from_raw_svg_cursor(raw_cursors.pirate, size, color_info).unwrap(),
        hand:                   RawCursor::from_raw_svg_cursor(raw_cursors.hand, size, color_info).unwrap(),
        arrow_right:            RawCursor::from_raw_svg_cursor(raw_cursors.arrow_right, size, color_info).unwrap(),
        mirrored_pointer:       RawCursor::from_raw_svg_cursor(raw_cursors.mirrored_pointer, size, color_info).unwrap(),
        arrow_right_stop:       RawCursor::from_raw_svg_cursor(raw_cursors.arrow_right_stop, size, color_info).unwrap(),
        resize_nesw:            RawCursor::from_raw_svg_cursor(raw_cursors.resize_nesw, size, color_info).unwrap(),
        resize_nwse:            RawCursor::from_raw_svg_cursor(raw_cursors.resize_nwse, size, color_info).unwrap(),
        resize_horizontal:      RawCursor::from_raw_svg_cursor(raw_cursors.resize_horizontal, size, color_info).unwrap(),
        resize_vertical:        RawCursor::from_raw_svg_cursor(raw_cursors.resize_vertical, size, color_info).unwrap(),
        text:                   RawCursor::from_raw_svg_cursor(raw_cursors.text, size, color_info).unwrap(),
        arrow_top_left:         RawCursor::from_raw_svg_cursor(raw_cursors.arrow_top_left, size, color_info).unwrap(),
        arrow_top_right:        RawCursor::from_raw_svg_cursor(raw_cursors.arrow_top_right, size, color_info).unwrap(),
        arrow_top_stop:         RawCursor::from_raw_svg_cursor(raw_cursors.arrow_top_stop, size, color_info).unwrap(),
        arrow_up:               RawCursor::from_raw_svg_cursor(raw_cursors.arrow_up, size, color_info).unwrap(),
        vertical_text:          RawCursor::from_raw_svg_cursor(raw_cursors.vertical_text, size, color_info).unwrap(),
        zoom_in:                RawCursor::from_raw_svg_cursor(raw_cursors.zoom_in, size, color_info).unwrap(),
        zoom_out:               RawCursor::from_raw_svg_cursor(raw_cursors.zoom_out, size, color_info).unwrap(),
    };

    final_cursors
}
