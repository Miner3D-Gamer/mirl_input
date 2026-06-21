use std::slice::SliceIndex;

// /// Takes in X cursors and generates the missing ones based on the closest available
// pub fn get_missing_cursor_sizes(images: &[(&[u8], (usize, usize))]) -> [u32; 8] {
//     todo!()
// }

#[must_use]
/// Create a bmp out of the given data
pub fn create_bmp(data: &[u32], size: (usize, usize)) -> Option<Vec<u8>> {
    if size.0 > i32::MAX as usize || size.1 > i32::MAX as usize {
        return None;
    }
    if data.len() >= size.0 * size.1 {
        return None;
    }
    let mut bmp_buffer: Vec<u8> = Vec::new();

    let width: u32 = size.0 as u32;
    let height: u32 = size.1 as u32;

    // BMP File Header (14 bytes)
    bmp_buffer.extend(&[0x42, 0x4D]); // "BM" signature

    let row_stride = (width * 32).div_ceil(32) * 4;
    let pixel_array_size = row_stride * height;
    let bmp_header_size = 40;
    let file_header_size = 14;
    let file_size: u32 = file_header_size + bmp_header_size + pixel_array_size;
    let pixel_data_offset = file_header_size + bmp_header_size;

    bmp_buffer.extend(&file_size.to_le_bytes()); // File size
    bmp_buffer.extend(&[0x00, 0x00]); // Reserved
    bmp_buffer.extend(&[0x00, 0x00]); // Reserved
    bmp_buffer.extend(&pixel_data_offset.to_le_bytes()); // Pixel data offset

    // BITMAPINFOHEADER (40 bytes)
    bmp_buffer.extend(&40u32.to_le_bytes()); // Header size
    bmp_buffer.extend(&(width as i32).to_le_bytes()); // Width
    bmp_buffer.extend(&(height as i32).to_le_bytes()); // Height (no x2 for BMP)
    bmp_buffer.extend(&1u16.to_le_bytes()); // Planes
    bmp_buffer.extend(&32u16.to_le_bytes()); // Bit count
    bmp_buffer.extend(&0u32.to_le_bytes()); // Compression
    bmp_buffer.extend(&pixel_array_size.to_le_bytes()); // Image size
    bmp_buffer.extend(&0u32.to_le_bytes()); // X pixels per meter
    bmp_buffer.extend(&0u32.to_le_bytes()); // Y pixels per meter
    bmp_buffer.extend(&0u32.to_le_bytes()); // Colors used
    bmp_buffer.extend(&0u32.to_le_bytes()); // Important colors

    bmp_buffer.reserve(size.0 * size.1 * 4);

    for y in (0..size.1).rev() {
        for x in 0..size.0 {
            let pixel = unsafe { (y * size.0 + x).get_unchecked(data) };

            bmp_buffer.extend(&<[u8; 4]>::from(
                (unsafe { *pixel }).unpack_u32_bgra(),
            ));
        }
    }

    Some(bmp_buffer)
}
#[must_use]
/// Create a Windows .cur file from the given data
/// TODO: Test if this works as intended
///
/// `hotspot` is the (x, y) coordinate of the cursor's hotspot
pub fn create_cur_simple(
    data: &[u32],
    size: (usize, usize),
    hotspot: (u8, u8),
) -> Option<Vec<u8>> {
    if !CursorResolution::is_valid_size(size) {
        return None;
    }
    if size.0 > u8::MAX as usize || size.1 > u8::MAX as usize {
        return None;
    }
    if data.len() < size.0 * size.1 {
        return None;
    }

    let width = size.0 as u8;
    let height = size.1 as u8;

    let row_stride = (u32::from(width) * 32).div_ceil(32) * 4;
    let pixel_array_size = row_stride * u32::from(height);
    let bmp_header_size: u32 = 40;
    let and_mask_size = u32::from(height) * (u32::from(width).div_ceil(32) * 4);
    let size_in_bytes = bmp_header_size + pixel_array_size + and_mask_size;
    let image_data_offset: u32 = 6 + 16;

    let mut cur_buffer: Vec<u8> = Vec::new();

    // ICONDIR (6 bytes)
    cur_buffer.extend(&[0x00, 0x00]); // Reserved
    cur_buffer.extend(&[0x02, 0x00]); // Image type (2 = cursor)
    cur_buffer.extend(&[0x01, 0x00]); // Number of images

    // ICONDIRENTRY (16 bytes)
    cur_buffer.push(width); // Width
    cur_buffer.push(height); // Height
    cur_buffer.push(0); // Color count (0 for 32bpp)
    cur_buffer.push(0); // Reserved
    cur_buffer.extend(&hotspot.0.to_le_bytes()); // Hotspot X
    cur_buffer.extend(&hotspot.1.to_le_bytes()); // Hotspot Y
    cur_buffer.extend(&size_in_bytes.to_le_bytes()); // Image data size
    cur_buffer.extend(&image_data_offset.to_le_bytes()); // Image data offset

    // BITMAPINFOHEADER (40 bytes)
    cur_buffer.extend(&40u32.to_le_bytes()); // Header size
    cur_buffer.extend(&i32::from(width).to_le_bytes()); // Width
    cur_buffer.extend(&(2 * i32::from(height)).to_le_bytes()); // Height (x2 for AND mask)
    cur_buffer.extend(&1u16.to_le_bytes()); // Planes
    cur_buffer.extend(&32u16.to_le_bytes()); // Bit count
    cur_buffer.extend(&0u32.to_le_bytes()); // Compression
    cur_buffer.extend(&0u32.to_le_bytes()); // Image size (0 = uncompressed)
    cur_buffer.extend(&0u32.to_le_bytes()); // X pixels per meter
    cur_buffer.extend(&0u32.to_le_bytes()); // Y pixels per meter
    cur_buffer.extend(&0u32.to_le_bytes()); // Colors used
    cur_buffer.extend(&0u32.to_le_bytes()); // Important colors

    // Pixel data (bottom-up, BGRA)
    cur_buffer.reserve(size.0 * size.1 * 4);
    for y in (0..size.1).rev() {
        for x in 0..size.0 {
            let pixel = data[y * size.0 + x];
            cur_buffer.extend(&<[u8; 4]>::from((pixel).unpack_u32_bgra()));
        }
    }

    // AND mask (all zero = fully visible)
    cur_buffer.extend(vec![0u8; and_mask_size as usize]);

    Some(cur_buffer)
}
#[must_use]
/// Create a Windows .cur file from the given data (with support for multiple cursor sizes)
/// TODO: Test if this works as intended
///
/// Each entry in `images` is `(data, (width, height))`.
/// `hotspot` is the (x, y) coordinate of the cursor's hotspot, shared across all images.
pub fn create_cur<'a, T: RawCursorFrameContainer>(
    cursor: &'a T,
) -> Option<Vec<u8>> {
    #[allow(unreachable_pub)]
    pub struct ImageMeta {
        width: u8,
        height: u8,
        size_in_bytes: u32,
        data_offset: u32,
    }

    let images: Vec<&'a RawCursor> = cursor.get_all_cursors_ref();

    if images.is_empty() {
        return None;
    }

    // // Validate all images upfront
    // for (data, (w, h)) in images {
    //     if *w > u8::MAX as usize || *h > u8::MAX as usize {
    //         return None;
    //     }
    //     if data.len() < w * h {
    //         return None;
    //     }
    // }

    let image_count = images.len() as u16;
    let mut cur_buffer: Vec<u8> = Vec::new();

    // ICONDIR (6 bytes)
    cur_buffer.extend(&[0x00, 0x00]); // Reserved
    cur_buffer.extend(&[0x02, 0x00]); // Image type (2 = cursor)
    cur_buffer.extend(&image_count.to_le_bytes()); // Number of images

    let mut image_meta: Vec<ImageMeta> = Vec::with_capacity(images.len());
    // Image data starts after ICONDIR (6) + all ICONDIRENTRYs (16 each)
    let mut data_offset: u32 = 6 + 16 * u32::from(image_count);

    for cursor in &images {
        let width = cursor.image.width as u8;
        let height = cursor.image.height as u8;
        let row_stride = (u32::from(width) * 32).div_ceil(32) * 4;
        let pixel_array_size = row_stride * u32::from(height);
        let bmp_header_size: u32 = 40;
        let and_mask_size =
            u32::from(height) * (u32::from(width).div_ceil(32) * 4);
        let size_in_bytes = bmp_header_size + pixel_array_size + and_mask_size;

        image_meta.push(ImageMeta {
            width,
            height,
            size_in_bytes,
            data_offset,
        });
        data_offset += size_in_bytes;
    }
    let hotspot = cursor.get_hotspot();

    // ICONDIRENTRYs (16 bytes each)
    for meta in &image_meta {
        cur_buffer.push(meta.width);
        cur_buffer.push(meta.height);
        cur_buffer.push(0); // Color count (0 for 32bpp)
        cur_buffer.push(0); // Reserved
        cur_buffer.extend(&hotspot.0.to_le_bytes()); // Hotspot X
        cur_buffer.extend(&hotspot.1.to_le_bytes()); // Hotspot Y
        cur_buffer.extend(&meta.size_in_bytes.to_le_bytes()); // Image data size
        cur_buffer.extend(&meta.data_offset.to_le_bytes()); // Image data offset
    }

    // Image data for each entry
    for (cursor_data, meta) in images.iter().zip(&image_meta) {
        let width = meta.width;
        let height = meta.height;
        let and_mask_size =
            u32::from(height) * (u32::from(width).div_ceil(32) * 4);

        // BITMAPINFOHEADER (40 bytes)
        cur_buffer.extend(&40u32.to_le_bytes());
        cur_buffer.extend(&i32::from(width).to_le_bytes());
        cur_buffer.extend(&(2 * i32::from(height)).to_le_bytes()); // x2 for AND mask
        cur_buffer.extend(&1u16.to_le_bytes()); // Planes
        cur_buffer.extend(&32u16.to_le_bytes()); // Bit count
        cur_buffer.extend(&0u32.to_le_bytes()); // Compression
        cur_buffer.extend(&0u32.to_le_bytes()); // Image size (0 = uncompressed)
        cur_buffer.extend(&0u32.to_le_bytes()); // X pixels per meter
        cur_buffer.extend(&0u32.to_le_bytes()); // Y pixels per meter
        cur_buffer.extend(&0u32.to_le_bytes()); // Colors used
        cur_buffer.extend(&0u32.to_le_bytes()); // Important colors

        // Pixel data (bottom-up, BGRA)
        cur_buffer
            .reserve(cursor_data.image.width * cursor_data.image.height * 4);
        for y in (0..cursor_data.image.height).rev() {
            for x in 0..cursor_data.image.width {
                let pixel = cursor_data.image[y * cursor_data.image.width + x];
                cur_buffer.extend(&<[u8; 4]>::from((pixel).unpack_u32_bgra()));
            }
        }
        // TODO: Add this debug feature
        // #[cfg(not(feature = "cursor_show_hotspot"))]

        // AND mask (all zero = fully visible)
        cur_buffer.extend(vec![0u8; and_mask_size as usize]);
    }

    Some(cur_buffer)
}
#[allow(clippy::type_complexity)]
/// Create a Windows .ani (animated cursor) file.
/// TODO:
/// - Test if this works as intended
/// - See if there is more we can configure
///
/// # Input:
///
/// Frames [
///     Resolutions(ColorData, Size)
///     ]
///
/// - `frames`: each frame is a slice of `(pixel_data, (width, height))` images
///   (multiple sizes per frame, just like `create_cur`).
/// - `hotspot`: the (x, y) hotspot, shared across all frames.
/// - `frame_rate`: display rate in jiffies (1 jiffy = 1/60 s). Applied to every frame.
///
/// # Errors
/// When am image could not be converted, its idx will be returned
pub fn create_ani(
    cursor: &AnimatedRawCursorWithResolution,
) -> Result<Vec<u8>, usize> {
    // Helper: write a RIFF chunk  →  FourCC (4 bytes) + size (4 LE) + data
    fn chunk(tag: [u8; 4], data: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(8 + data.len());
        v.extend(tag);
        v.extend(&(data.len() as u32).to_le_bytes());
        v.extend(data);
        // RIFF chunks must be word-aligned; pad with a zero byte if needed.
        if data.len().is_multiple_of(2) {
            v.push(0);
        }
        v
    }

    // Helper: write a LIST chunk  →  'LIST' + size + list-type FourCC + children
    fn list_chunk(list_type: [u8; 4], children: &[u8]) -> Vec<u8> {
        let data_len = 4 + children.len(); // list-type + children
        let mut v = Vec::with_capacity(8 + data_len);
        v.extend(b"LIST");
        v.extend(&(data_len as u32).to_le_bytes());
        v.extend(list_type);
        v.extend(children);
        if children.len().is_multiple_of(2) {
            v.push(0);
        }
        v
    }
    let frames = cursor.to_raw_cursor_with_resolution();

    // if frames.is_empty() {
    //     return Err(usize::MAX);
    // }
    // Build each frame as a .cur blob up front so we know their sizes.
    let mut cur_blobs: Vec<Vec<u8>> = Vec::with_capacity(frames.len());
    for (idx, images) in frames.iter().enumerate() {
        cur_blobs.push(create_cur(images).map_or_else(|| Err(idx), Ok)?);
    }
    let frame_rate = 1u32; // How many frames are held -> 1=60fps,  2=30fps, 3=15fps.

    // anih — ANI HEADER (36 bytes, all LE u32)
    let n_frames = cur_blobs.len() as u32;
    let mut anih_data = Vec::with_capacity(36);
    anih_data.extend(&36u32.to_le_bytes()); // cbSizeof
    anih_data.extend(&n_frames.to_le_bytes()); // cFrames
    anih_data.extend(&n_frames.to_le_bytes()); // cSteps  (= cFrames: play each frame once)
    anih_data.extend(&0u32.to_le_bytes()); // cx      (unused when AF_ICON set)
    anih_data.extend(&0u32.to_le_bytes()); // cy
    anih_data.extend(&0u32.to_le_bytes()); // cBitCount
    anih_data.extend(&0u32.to_le_bytes()); // cPlanes
    anih_data.extend(&frame_rate.to_le_bytes()); // jifRate
    anih_data.extend(&0x01u32.to_le_bytes()); // fl = AF_ICON (frames are CUR/ICO resources)
    let anih_chunk = chunk(*b"anih", &anih_data);

    // LIST('fram'  icon(...)  icon(...)  ...)
    let mut fram_children: Vec<u8> = Vec::new();
    for blob in &cur_blobs {
        fram_children.extend(chunk(*b"icon", blob));
    }
    let fram_chunk = list_chunk(*b"fram", &fram_children);

    // RIFF('ACON'  anih  LIST('fram' ...))
    let mut riff_data: Vec<u8> = Vec::new();
    riff_data.extend(b"ACON"); // RIFF form type
    riff_data.extend(&anih_chunk);
    riff_data.extend(&fram_chunk);

    let mut ani_buffer: Vec<u8> = Vec::with_capacity(8 + riff_data.len());
    ani_buffer.extend(b"RIFF");
    ani_buffer.extend(&(riff_data.len() as u32).to_le_bytes());
    ani_buffer.extend(&riff_data);

    Ok(ani_buffer)
}

use mirl_buffer::prelude::*;
use mirl_graphics::u32_color_casting::UnpackColorIntoChannels;

use crate::{
    // graphics::u32_to_rgba_u8,
    mouse::{
        RawCursorFrameContainer,
        cursors::{
            AnimatedRawCursorWithResolution, CursorResolution, RawCursor,
        },
    },
};

// TODO:
// Make sure that the buffer size is a valid cursor size
// Automatically create smaller cursor sizes when possible so windows can choose which one to use
// Add support for .ani files (Animated .cur files)

/// Create a bitmap from the buffer
pub const trait BufferToBmp {
    /// Create a bitmap from the buffer
    fn create_bmp(&self) -> Option<Vec<u8>>;
}
impl<S: BufferData + BufferMetrics> BufferToBmp for S {
    /// Convert the image the buffer is holding into a bmp
    fn create_bmp(&self) -> Option<Vec<u8>> {
        create_bmp(self.data(), self.get_size())
    }
}

/// Create a windows .ico file from the buffer
pub const trait BufferToIco {
    /// Create a .ico file from the buffer
    fn create_ico(&self) -> Option<Vec<u8>>;
}
impl<S: BufferData + BufferMetrics> BufferToIco for S {
    fn create_ico(&self) -> Option<Vec<u8>> {
        let mut cur = create_cur_simple(self.data(), self.get_size(), (0, 0))?;
        unsafe { *cur.get_mut(6).unwrap_unchecked() = 0x01 };
        Some(cur)
    }
}
