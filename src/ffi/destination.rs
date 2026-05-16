#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_destination_new(
        page_handle: *mut c_void,
        x: f64,
        y: f64,
        out_destination: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_destination_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_destination_page(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_destination_set_zoom(handle: *mut c_void, value: f64);
    pub fn pdf_destination_compare(lhs_handle: *mut c_void, rhs_handle: *mut c_void) -> i32;
}
