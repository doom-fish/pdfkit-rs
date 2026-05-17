#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_page_overlay_view_new(
        width: f64,
        height: f64,
        out_view: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
