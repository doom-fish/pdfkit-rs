#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_goto_new(
        destination_handle: *mut c_void,
        out_action: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_goto_destination(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_action_goto_set_destination(
        handle: *mut c_void,
        destination_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_goto_type_string(handle: *mut c_void) -> *mut c_char;
}
