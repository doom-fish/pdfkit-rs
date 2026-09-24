use std::fmt;
use std::os::raw::c_void;
use std::ptr;

use doom_fish_utils::callback_context::CallbackContext;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::main_thread::MainThreadCell;
use crate::page::PdfPage;
use crate::page_overlay_view::PdfPageOverlayView;
use crate::view::PdfView;

/// Mirrors the `PDFPageOverlayViewProvider` callback surface.
pub trait PdfPageOverlayViewProvider: 'static {
    /// Mirrors the corresponding `PDFPageOverlayViewProvider` callback.
    fn overlay_view_for_page(
        &mut self,
        _view: PdfView,
        _page: PdfPage,
    ) -> Option<PdfPageOverlayView> {
        None
    }

    /// Mirrors the corresponding `PDFPageOverlayViewProvider` callback.
    fn will_display_overlay_view(
        &mut self,
        _view: PdfView,
        _overlay_view: PdfPageOverlayView,
        _page: PdfPage,
    ) {
    }

    /// Mirrors the corresponding `PDFPageOverlayViewProvider` callback.
    fn will_end_displaying_overlay_view(
        &mut self,
        _view: PdfView,
        _overlay_view: PdfPageOverlayView,
        _page: PdfPage,
    ) {
    }
}

type ProviderContext = CallbackContext<MainThreadCell<Box<dyn PdfPageOverlayViewProvider>>>;

/// Wraps `PDFPageOverlayViewProviderHandle`.
pub struct PdfPageOverlayViewProviderHandle {
    handle: ObjectHandle,
    context: ProviderContext,
}

impl PdfPageOverlayViewProviderHandle {
    /// Registers a Rust implementation of `PDFPageOverlayViewProvider`.
    pub fn new(provider: impl PdfPageOverlayViewProvider) -> Result<Self> {
        let context = ProviderContext::new(MainThreadCell::new(
            Box::new(provider) as Box<dyn PdfPageOverlayViewProvider>,
            "PdfPageOverlayViewProviderHandle",
        )?);
        let mut out_provider = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_overlay_view_provider_new(
                context.as_ptr(),
                Some(pdf_page_overlay_view_provider_overlay_trampoline),
                Some(pdf_page_overlay_view_provider_will_display_trampoline),
                Some(pdf_page_overlay_view_provider_will_end_displaying_trampoline),
                ProviderContext::RETAIN,
                ProviderContext::RELEASE,
                &raw mut out_provider,
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        let handle = crate::util::required_handle(out_provider, "PDFPageOverlayViewProvider")?;
        Ok(Self { handle, context })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for PdfPageOverlayViewProviderHandle {
    fn drop(&mut self) {
        self.context.deactivate();
    }
}

impl fmt::Debug for PdfPageOverlayViewProviderHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PdfPageOverlayViewProviderHandle")
            .finish_non_exhaustive()
    }
}

unsafe fn with_provider<R>(
    context: *mut c_void,
    site: &str,
    f: impl FnOnce(&mut dyn PdfPageOverlayViewProvider) -> R,
) -> Option<R> {
    unsafe {
        ProviderContext::with(context, site, |cell| {
            cell.with(|provider| f(provider.as_mut()))
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

/// Helper to convert a retained PDFPage pointer to a PdfPage.
///
/// # Safety
/// `handle` must be either null or a valid, retained pointer to a PDFPage object from Swift.
unsafe fn retained_page(handle: *mut c_void) -> Option<PdfPage> {
    // SAFETY: caller guarantees valid handle or null
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfPage::from_handle)
}

/// Helper to convert a retained PDFPageOverlayView pointer to a PdfPageOverlayView.
///
/// # Safety
/// `handle` must be either null or a valid, retained pointer to a PDFPageOverlayView object from Swift.
unsafe fn retained_overlay_view(handle: *mut c_void) -> Option<PdfPageOverlayView> {
    // SAFETY: caller guarantees valid handle or null
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfPageOverlayView::from_handle)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the provider's
/// `CallbackContext` pointer, and `view_handle` and `page_handle` must be retained PDFView and
/// PDFPage pointers from Swift (or null); both are released even when the provider is gone.
/// The returned pointer is a retained PDFPageOverlayView or null. Panics are caught to prevent
/// unwinding across the FFI boundary.
unsafe extern "C" fn pdf_page_overlay_view_provider_overlay_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    page_handle: *mut c_void,
) -> *mut c_void {
    let view = unsafe { retained_view(view_handle) };
    let page = unsafe { retained_page(page_handle) };
    let (Some(view), Some(page)) = (view, page) else {
        return ptr::null_mut();
    };
    unsafe {
        with_provider(context, "pdf_page_overlay_view_provider_overlay", |provider| {
            provider.overlay_view_for_page(view, page)
        })
    }
    .flatten()
    .map_or(ptr::null_mut(), PdfPageOverlayView::into_handle_ptr)
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the provider's
/// `CallbackContext` pointer, and `view_handle`, `overlay_view_handle` and `page_handle` must
/// be retained PDFView, PDFPageOverlayView and PDFPage pointers from Swift (or null); they are
/// released even when the provider is gone. Panics are caught to prevent unwinding across the
/// FFI boundary.
unsafe extern "C" fn pdf_page_overlay_view_provider_will_display_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    overlay_view_handle: *mut c_void,
    page_handle: *mut c_void,
) {
    let view = unsafe { retained_view(view_handle) };
    let overlay_view = unsafe { retained_overlay_view(overlay_view_handle) };
    let page = unsafe { retained_page(page_handle) };
    let (Some(view), Some(overlay_view), Some(page)) = (view, overlay_view, page) else {
        return;
    };
    let _ = unsafe {
        with_provider(context, "pdf_page_overlay_view_provider_will_display", |provider| {
            provider.will_display_overlay_view(view, overlay_view, page);
        })
    };
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the provider's
/// `CallbackContext` pointer, and `view_handle`, `overlay_view_handle` and `page_handle` must
/// be retained PDFView, PDFPageOverlayView and PDFPage pointers from Swift (or null); they are
/// released even when the provider is gone. Panics are caught to prevent unwinding across the
/// FFI boundary.
unsafe extern "C" fn pdf_page_overlay_view_provider_will_end_displaying_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    overlay_view_handle: *mut c_void,
    page_handle: *mut c_void,
) {
    let view = unsafe { retained_view(view_handle) };
    let overlay_view = unsafe { retained_overlay_view(overlay_view_handle) };
    let page = unsafe { retained_page(page_handle) };
    let (Some(view), Some(overlay_view), Some(page)) = (view, overlay_view, page) else {
        return;
    };
    let _ = unsafe {
        with_provider(
            context,
            "pdf_page_overlay_view_provider_will_end_displaying",
            |provider| {
                provider.will_end_displaying_overlay_view(view, overlay_view, page);
            },
        )
    };
}
