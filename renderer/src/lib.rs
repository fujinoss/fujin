//! arashi — the GPU renderer for Fujin.
//!
//! Vulkan primary, GLES fallback. Exposes 7 functions through a C ABI.
//! Called from the Zig engine, which is called from Android via JNI.

use core::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArashiCell {
    pub x: u16,
    pub y: u16,
    pub atlas_u: u16,
    pub atlas_v: u16,
    pub fg: u32,
    pub bg: u32,
    pub flags: u8,
    pub _pad: [u8; 3],
}

#[no_mangle]
pub extern "C" fn arashi_init(
    window: *mut c_void,
    width: u32,
    height: u32,
) -> i32 {
    let _ = (window, width, height);
    0
}

#[no_mangle]
pub extern "C" fn arashi_shutdown() {}

#[no_mangle]
pub extern "C" fn arashi_resize(width: u32, height: u32) {
    let _ = (width, height);
}

#[no_mangle]
pub extern "C" fn arashi_upload_atlas(
    data: *const u8,
    len: usize,
    atlas_w: u32,
    atlas_h: u32,
    format: u32,
) {
    let _ = (data, len, atlas_w, atlas_h, format);
}

#[no_mangle]
pub extern "C" fn arashi_begin_frame(clear_color: u32) {
    let _ = clear_color;
}

#[no_mangle]
pub extern "C" fn arashi_push_cells(
    cells: *const ArashiCell,
    count: usize,
    cols: u32,
    rows: u32,
    cell_w: f32,
    cell_h: f32,
) {
    let _ = (cells, count, cols, rows, cell_w, cell_h);
}

#[no_mangle]
pub extern "C" fn arashi_end_frame() {}
