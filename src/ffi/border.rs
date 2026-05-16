#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_border_new(out_border: *mut *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    pub fn pdf_border_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_border_set_style(
        handle: *mut c_void,
        raw_value: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_border_set_line_width(handle: *mut c_void, value: f64);
    pub fn pdf_border_set_dash_pattern(
        handle: *mut c_void,
        values: *const f64,
        len: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
