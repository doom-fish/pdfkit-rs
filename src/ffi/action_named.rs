#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_named_new(
        raw_name: i32,
        out_action: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_named_name_raw(handle: *mut c_void) -> i32;
    pub fn pdf_action_named_set_name(
        handle: *mut c_void,
        raw_name: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_named_type_string(handle: *mut c_void) -> *mut c_char;
}
