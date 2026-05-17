#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_action_reset_form_new(
        out_action: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_reset_form_fields_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_action_reset_form_set_fields_json(
        handle: *mut c_void,
        fields_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_action_reset_form_fields_included_are_cleared(handle: *mut c_void) -> i32;
    pub fn pdf_action_reset_form_set_fields_included_are_cleared(handle: *mut c_void, value: i32);
    pub fn pdf_action_reset_form_type_string(handle: *mut c_void) -> *mut c_char;
}
