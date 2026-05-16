#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_thumbnail_view_new(
        width: f64,
        height: f64,
        out_view: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_thumbnail_view_info_json(handle: *mut c_void) -> *mut c_char;
    pub fn pdf_thumbnail_view_pdf_view(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_thumbnail_view_set_pdf_view(
        handle: *mut c_void,
        pdf_view_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_thumbnail_view_set_thumbnail_size(handle: *mut c_void, width: f64, height: f64);
    pub fn pdf_thumbnail_view_set_maximum_number_of_columns(handle: *mut c_void, value: u64);
    pub fn pdf_thumbnail_view_set_allows_dragging(handle: *mut c_void, value: i32);
    pub fn pdf_thumbnail_view_set_allows_multiple_selection(handle: *mut c_void, value: i32);
    pub fn pdf_thumbnail_view_selected_page_count(handle: *mut c_void) -> u64;
    pub fn pdf_thumbnail_view_selected_page_at(handle: *mut c_void, index: u64) -> *mut c_void;
}
