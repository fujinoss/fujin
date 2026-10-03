//! arashi — GPU renderer for Fujin.
//!
//! Vulkan primary, GLES fallback. C ABI on top, called from the Zig engine.

pub mod color;
pub mod util;

pub use color::{Color, Palette};
pub use util::{ArashiError, FrameTimer, Result};

use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn arashi_init(
    _window: *mut c_void,
    _width: u32,
    _height: u32,
) -> i32 {
    util::log::init();
    0
}

#[no_mangle]
pub extern "C" fn arashi_shutdown() {}

#[no_mangle]
pub extern "C" fn arashi_resize(_width: u32, _height: u32) {}

#[no_mangle]
pub extern "C" fn arashi_upload_atlas(
    _data: *const u8,
    _len: usize,
    _atlas_w: u32,
    _atlas_h: u32,
    _format: u32,
) {
}

#[no_mangle]
pub extern "C" fn arashi_begin_frame(_clear_color: u32) {}

#[no_mangle]
pub extern "C" fn arashi_push_cells(
    _cells: *const u8,
    _count: usize,
    _cols: u32,
    _rows: u32,
    _cell_w: f32,
    _cell_h: f32,
) {
}

#[no_mangle]
pub extern "C" fn arashi_end_frame() {}
