#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn pdf_object_retain(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_object_release(handle: *mut c_void);

    pub fn pdf_document_new_with_url(
        path: *const c_char,
        out_document: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_new_with_data(
        bytes: *const u8,
        len: usize,
        out_document: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_document_attributes_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_document_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_document_page_count(handle: *mut c_void) -> u64;
    pub fn pdf_document_page_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_document_outline_root(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_document_unlock(handle: *mut c_void, password: *const c_char) -> i32;
    pub fn pdf_document_write_to_url(
        handle: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn pdf_page_label_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_page_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_page_number_of_characters(handle: *mut c_void) -> u64;
    pub fn pdf_page_rotation(handle: *mut c_void) -> i32;
    pub fn pdf_page_bounds(
        handle: *mut c_void,
        display_box: i32,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );
    pub fn pdf_page_annotation_count(handle: *mut c_void) -> u64;
    pub fn pdf_page_annotation_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_page_selection_for_range(
        handle: *mut c_void,
        location: u64,
        length: u64,
    ) -> *mut c_void;

    pub fn pdf_selection_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_selection_page_count(handle: *mut c_void) -> u64;
    pub fn pdf_selection_page_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_selection_bounds_for_page(
        selection_handle: *mut c_void,
        page_handle: *mut c_void,
        out_x: *mut f64,
        out_y: *mut f64,
        out_width: *mut f64,
        out_height: *mut f64,
    );

    pub fn pdf_outline_label_string(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_outline_child_count(handle: *mut c_void) -> u64;
    pub fn pdf_outline_child_at(handle: *mut c_void, index: u64) -> *mut c_void;

    pub fn pdf_annotation_info_json(handle: *mut c_void) -> *mut c_char;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const NULL_RESULT: i32 = -2;
    pub const FRAMEWORK: i32 = -3;
}
