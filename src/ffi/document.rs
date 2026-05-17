#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn pdf_document_new(
        out_document: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
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
    pub fn pdf_document_index_for_page(
        document_handle: *mut c_void,
        page_handle: *mut c_void,
    ) -> u64;
    pub fn pdf_document_outline_root(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_document_set_outline_root(
        document_handle: *mut c_void,
        outline_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_outline_item_for_selection(
        document_handle: *mut c_void,
        selection_handle: *mut c_void,
    ) -> *mut c_void;
    pub fn pdf_document_selection_for_entire_document(handle: *mut c_void) -> *mut c_void;
    pub fn pdf_document_selection_from_pages_points(
        document_handle: *mut c_void,
        start_page_handle: *mut c_void,
        start_x: f64,
        start_y: f64,
        end_page_handle: *mut c_void,
        end_x: f64,
        end_y: f64,
    ) -> *mut c_void;
    pub fn pdf_document_selection_from_pages_points_with_granularity(
        document_handle: *mut c_void,
        start_page_handle: *mut c_void,
        start_x: f64,
        start_y: f64,
        end_page_handle: *mut c_void,
        end_x: f64,
        end_y: f64,
        granularity: u64,
    ) -> *mut c_void;
    pub fn pdf_document_selection_from_pages_characters(
        document_handle: *mut c_void,
        start_page_handle: *mut c_void,
        start_character: u64,
        end_page_handle: *mut c_void,
        end_character: u64,
    ) -> *mut c_void;
    pub fn pdf_document_unlock(handle: *mut c_void, password: *const c_char) -> i32;
    pub fn pdf_document_set_delegate(
        handle: *mut c_void,
        delegate_handle: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_write_to_url(
        handle: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_write_to_url_with_options(
        handle: *mut c_void,
        path: *const c_char,
        options_json: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_insert_page(
        document_handle: *mut c_void,
        page_handle: *mut c_void,
        index: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_remove_page_at(
        document_handle: *mut c_void,
        index: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn pdf_document_exchange_pages(
        document_handle: *mut c_void,
        index_a: u64,
        index_b: u64,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
