#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type PdfDocumentDelegateNotificationCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, raw_notification: i32)>;
pub type PdfDocumentDelegateMatchCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, selection_handle: *mut c_void)>;
pub type PdfDocumentDelegatePageClassNameCallback =
    Option<unsafe extern "C" fn(context: *mut c_void) -> *mut c_char>;
pub type PdfDocumentDelegateAnnotationClassNameCallback = Option<
    unsafe extern "C" fn(context: *mut c_void, annotation_type: *const c_char) -> *mut c_char,
>;

unsafe extern "C" {
    pub fn pdf_document_delegate_new(
        context: *mut c_void,
        notification_callback: PdfDocumentDelegateNotificationCallback,
        match_callback: PdfDocumentDelegateMatchCallback,
        page_class_name_callback: PdfDocumentDelegatePageClassNameCallback,
        annotation_class_name_callback: PdfDocumentDelegateAnnotationClassNameCallback,
        out_delegate: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}
