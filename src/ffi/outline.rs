#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_outline_new(out_outline: *mut *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    pub fn pdf_outline_label_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_outline_set_label(
        handle: *mut c_void,
        value: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_outline_child_count(handle: *mut c_void) -> u64;
    pub fn pdf_outline_child_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_outline_insert_child(
        handle: *mut c_void,
        child_handle: *mut c_void,
        index: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_outline_remove_from_parent(handle: *mut c_void);
    pub fn pdf_outline_index(handle: *mut c_void) -> u64;
    pub fn pdf_outline_is_open(handle: *mut c_void) -> i32;
    pub fn pdf_outline_set_open(handle: *mut c_void, value: i32);
    pub fn pdf_outline_destination(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_outline_set_destination(
        handle: *mut c_void,
        destination_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_outline_action_url(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_outline_set_action_url(
        handle: *mut c_void,
        action_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_outline_action_goto(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_outline_set_action_goto(
        handle: *mut c_void,
        action_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_outline_parent(handle: *mut c_void) -> *mut c_void;
}
