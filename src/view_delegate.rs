use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_void};
use std::ptr;

use doom_fish_utils::callback_context::CallbackContext;

use crate::action_remote_goto::PdfActionRemoteGoTo;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::main_thread::MainThreadCell;
use crate::view::PdfView;

/// Mirrors the `PDFViewDelegate` callback surface.
pub trait PdfViewDelegate: 'static {
    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn handle_link_click(&mut self, _view: PdfView, _url: &str) -> bool {
        false
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn will_change_scale_factor(&mut self, _view: PdfView, scale_factor: f64) -> f64 {
        scale_factor.clamp(0.1, 10.0)
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn print_job_title(&mut self, _view: PdfView) -> Option<String> {
        None
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn perform_print(&mut self, _view: PdfView) -> bool {
        false
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn perform_find(&mut self, _view: PdfView) -> bool {
        false
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn perform_go_to_page(&mut self, _view: PdfView) -> bool {
        false
    }

    /// Mirrors the corresponding `PDFViewDelegate` callback.
    fn open_pdf_for_remote_goto_action(
        &mut self,
        _view: PdfView,
        _action: PdfActionRemoteGoTo,
    ) -> bool {
        false
    }
}

type DelegateContext = CallbackContext<MainThreadCell<Box<dyn PdfViewDelegate>>>;

/// Wraps `PDFViewDelegateHandle`.
pub struct PdfViewDelegateHandle {
    handle: ObjectHandle,
    context: DelegateContext,
}

impl PdfViewDelegateHandle {
    /// Registers a Rust implementation of `PDFViewDelegate`.
    pub fn new(delegate: impl PdfViewDelegate) -> Result<Self> {
        let context = DelegateContext::new(MainThreadCell::new(
            Box::new(delegate) as Box<dyn PdfViewDelegate>,
            "PdfViewDelegateHandle",
        )?);
        let mut out_delegate = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_delegate_new(
                context.as_ptr(),
                Some(pdf_view_delegate_link_click_trampoline),
                Some(pdf_view_delegate_scale_factor_trampoline),
                Some(pdf_view_delegate_print_job_title_trampoline),
                Some(pdf_view_delegate_perform_print_trampoline),
                Some(pdf_view_delegate_perform_find_trampoline),
                Some(pdf_view_delegate_perform_go_to_page_trampoline),
                Some(pdf_view_delegate_remote_goto_trampoline),
                DelegateContext::RETAIN,
                DelegateContext::RELEASE,
                &raw mut out_delegate,
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        let handle = crate::util::required_handle(out_delegate, "PDFViewDelegate")?;
        Ok(Self { handle, context })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for PdfViewDelegateHandle {
    fn drop(&mut self) {
        self.context.deactivate();
    }
}

impl fmt::Debug for PdfViewDelegateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PdfViewDelegateHandle")
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

unsafe fn with_delegate<R>(
    context: *mut c_void,
    site: &str,
    f: impl FnOnce(&mut dyn PdfViewDelegate) -> R,
) -> Option<R> {
    unsafe {
        DelegateContext::with(context, site, |cell| {
            cell.with(|delegate| f(delegate.as_mut()))
        })
    }
    .flatten()
}

/// Helper to convert a retained PDFView pointer to a PdfView.
///
/// # Safety
/// `handle` must be either null or a valid, retained pointer to a PDFView object from Swift.
unsafe fn retained_view(handle: *mut c_void) -> Option<PdfView> {
    // SAFETY: caller guarantees valid handle or null
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfView::from_handle)
}

/// Helper to convert a retained PDFActionRemoteGoTo pointer to a PdfActionRemoteGoTo.
///
/// # Safety
/// `handle` must be either null or a valid, retained pointer to a PDFActionRemoteGoTo object from Swift.
unsafe fn retained_remote_goto_action(handle: *mut c_void) -> Option<PdfActionRemoteGoTo> {
    // SAFETY: caller guarantees valid handle or null
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfActionRemoteGoTo::from_handle)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, `view_handle` must be a retained PDFView pointer from Swift (or
/// null), and `url` must be a valid C string (or null). Panics are caught to prevent unwinding
/// across the FFI boundary.
unsafe extern "C" fn pdf_view_delegate_link_click_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    url: *const c_char,
) -> i32 {
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return 0;
    };
    if url.is_null() {
        return 0;
    }
    unsafe {
        with_delegate(context, "pdf_view_delegate_link_click", |delegate| {
            let url = CStr::from_ptr(url).to_string_lossy();
            delegate.handle_link_click(view, &url)
        })
    }
    .map_or(0, i32::from)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` must be a retained PDFView pointer from Swift
/// (or null). Panics are caught to prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_view_delegate_scale_factor_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    scale_factor: f64,
) -> f64 {
    let fallback = scale_factor.clamp(0.1, 10.0);
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return fallback;
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_scale_factor", |delegate| {
            delegate.will_change_scale_factor(view, scale_factor)
        })
    }
    .unwrap_or(fallback)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` must be a retained PDFView pointer from Swift
/// (or null). The returned pointer must be freed by the Swift caller.
unsafe extern "C" fn pdf_view_delegate_print_job_title_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> *mut c_char {
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return ptr::null_mut();
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_print_job_title", |delegate| {
            duplicate_string(delegate.print_job_title(view))
        })
    }
    .unwrap_or(ptr::null_mut())
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` must be a retained PDFView pointer from Swift
/// (or null).
unsafe extern "C" fn pdf_view_delegate_perform_print_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return 0;
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_perform_print", |delegate| {
            delegate.perform_print(view)
        })
    }
    .map_or(0, i32::from)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` must be a retained PDFView pointer from Swift
/// (or null).
unsafe extern "C" fn pdf_view_delegate_perform_find_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return 0;
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_perform_find", |delegate| {
            delegate.perform_find(view)
        })
    }
    .map_or(0, i32::from)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` must be a retained PDFView pointer from Swift
/// (or null).
unsafe extern "C" fn pdf_view_delegate_perform_go_to_page_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    let Some(view) = (unsafe { retained_view(view_handle) }) else {
        return 0;
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_perform_go_to_page", |delegate| {
            delegate.perform_go_to_page(view)
        })
    }
    .map_or(0, i32::from)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `view_handle` and `action_handle` must be retained pointers
/// from Swift (or null); both are released even when the delegate is gone.
unsafe extern "C" fn pdf_view_delegate_remote_goto_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    action_handle: *mut c_void,
) -> i32 {
    let view = unsafe { retained_view(view_handle) };
    let action = unsafe { retained_remote_goto_action(action_handle) };
    let (Some(view), Some(action)) = (view, action) else {
        return 0;
    };
    unsafe {
        with_delegate(context, "pdf_view_delegate_remote_goto", |delegate| {
            delegate.open_pdf_for_remote_goto_action(view, action)
        })
    }
    .map_or(0, i32::from)
}
