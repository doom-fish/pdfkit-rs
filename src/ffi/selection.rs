#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_selection_new(
        document_handle: *mut c_void,
        out_selection: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
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
    pub fn pdf_selection_number_of_text_ranges_on_page(
        selection_handle: *mut c_void,
        page_handle: *mut c_void,
    ) -> u64;
    pub fn pdf_selection_range_at_index_on_page(
        selection_handle: *mut c_void,
        index: u64,
        page_handle: *mut c_void,
        out_location: *mut u64,
        out_length: *mut u64,
    ) -> i32;
    pub fn pdf_selection_selections_by_line_count(handle: *mut c_void) -> u64;
    pub fn pdf_selection_selection_by_line_at(handle: *mut c_void, index: u64) -> *mut c_void;
    pub fn pdf_selection_add_selection(
        selection_handle: *mut c_void,
        other_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_selection_extend_at_end(handle: *mut c_void, amount: i64);
    pub fn pdf_selection_extend_at_start(handle: *mut c_void, amount: i64);
    pub fn pdf_selection_extend_for_line_boundaries(handle: *mut c_void);
}
