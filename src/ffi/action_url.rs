#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_url_new(
        url_string: *const c_char,
        out_action: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_url_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_action_url_set_url(
        handle: *mut c_void,
        url_string: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_url_type_string(handle: *mut c_void) -> *mut c_char;
}
