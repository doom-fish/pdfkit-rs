use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::notifications::PdfDocumentNotification;
use crate::selection::PdfSelection;

/// Mirrors the `PDFDocumentDelegate` callback surface.
pub trait PdfDocumentDelegate: 'static {
    /// Mirrors the corresponding `PDFDocumentDelegate` callback.
    fn handle_notification(&mut self, _notification: PdfDocumentNotification) {}

    /// Mirrors the corresponding `PDFDocumentDelegate` callback.
    fn did_match_string(&mut self, _instance: PdfSelection) {}

    /// Mirrors the corresponding `PDFDocumentDelegate` callback.
    fn page_class_name(&mut self) -> Option<String> {
        None
    }

    /// Mirrors the corresponding `PDFDocumentDelegate` callback.
    fn annotation_class_name(&mut self, _annotation_type: &str) -> Option<String> {
        None
    }
}

struct DelegateState {
    delegate: Box<dyn PdfDocumentDelegate>,
}

/// Wraps `PDFDocumentDelegateHandle`.
pub struct PdfDocumentDelegateHandle {
    handle: ObjectHandle,
    _state: Box<DelegateState>,
}

impl PdfDocumentDelegateHandle {
    /// Registers a Rust implementation of `PDFDocumentDelegate`.
    pub fn new(delegate: impl PdfDocumentDelegate) -> Result<Self> {
        let mut state = Box::new(DelegateState {
            delegate: Box::new(delegate),
        });
        let context = ptr::addr_of_mut!(*state).cast::<c_void>();
        let mut out_delegate = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_delegate_new(
                context,
                Some(pdf_document_delegate_notification_trampoline),
                Some(pdf_document_delegate_match_trampoline),
                Some(pdf_document_delegate_page_class_name_trampoline),
                Some(pdf_document_delegate_annotation_class_name_trampoline),
                &mut out_delegate,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self {
            handle: crate::util::required_handle(out_delegate, "PDFDocumentDelegate")?,
            _state: state,
        })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl fmt::Debug for PdfDocumentDelegateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PdfDocumentDelegateHandle")
            .finish_non_exhaustive()
    }
}

fn duplicate_string(value: Option<String>) -> *mut c_char {
    value
        .and_then(|value| CString::new(value).ok())
        .map_or(ptr::null_mut(), |value| unsafe {
            libc::strdup(value.as_ptr())
        })
}

/// # Safety
/// The caller must ensure that `context` is either null or a valid pointer to `DelegateState`
/// that was obtained from `Box::into_raw()` and has not yet been freed.
unsafe fn delegate_state(context: *mut c_void) -> Option<&'static mut DelegateState> {
    context.cast::<DelegateState>().as_mut()
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. The caller must pass a valid, non-null
/// `context` pointer that points to `DelegateState`. Panics are caught to prevent unwinding
/// across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_notification_trampoline(
    context: *mut c_void,
    raw_notification: i32,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: caller is responsible for providing valid context pointer
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return;
        };
        let Some(notification) = PdfDocumentNotification::from_raw(raw_notification) else {
            return;
        };
        state.delegate.handle_notification(notification);
    }));
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. The caller must pass a valid, non-null
/// `context` pointer that points to `DelegateState`, and `selection_handle` must be a
/// retained PDFSelection pointer from Swift (or null). Panics are caught to prevent unwinding
/// across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_match_trampoline(
    context: *mut c_void,
    selection_handle: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: caller is responsible for providing valid context and selection_handle pointers
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return;
        };
        let Some(handle) = (unsafe { ObjectHandle::from_retained_ptr(selection_handle) }) else {
            return;
        };
        state
            .delegate
            .did_match_string(PdfSelection::from_handle(handle));
    }));
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. The caller must pass a valid, non-null
/// `context` pointer that points to `DelegateState`. The returned pointer must be freed by
/// the Swift caller. Panics are caught to prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_page_class_name_trampoline(
    context: *mut c_void,
) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: caller is responsible for providing valid context pointer
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return ptr::null_mut();
        };
        duplicate_string(state.delegate.page_class_name())
    }))
    .unwrap_or(ptr::null_mut())
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. The caller must pass a valid, non-null
/// `context` pointer that points to `DelegateState`, and `annotation_type` must be either
/// null or a valid C string pointer. The returned pointer must be freed by the Swift caller.
/// Panics are caught to prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_annotation_class_name_trampoline(
    context: *mut c_void,
    annotation_type: *const c_char,
) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: caller is responsible for providing valid context and annotation_type pointers
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return ptr::null_mut();
        };
        let Some(annotation_type) = (!annotation_type.is_null()).then(|| unsafe {
            // SAFETY: checked for null above; Swift guarantees valid C string
            CStr::from_ptr(annotation_type)
                .to_string_lossy()
                .into_owned()
        }) else {
            return ptr::null_mut();
        };
        duplicate_string(state.delegate.annotation_class_name(&annotation_type))
    }))
    .unwrap_or(ptr::null_mut())
}
