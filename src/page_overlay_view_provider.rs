use std::fmt;
use std::os::raw::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::page_overlay_view::PdfPageOverlayView;
use crate::view::PdfView;

pub trait PdfPageOverlayViewProvider: 'static {
    fn overlay_view_for_page(
        &mut self,
        _view: PdfView,
        _page: PdfPage,
    ) -> Option<PdfPageOverlayView> {
        None
    }

    fn will_display_overlay_view(
        &mut self,
        _view: PdfView,
        _overlay_view: PdfPageOverlayView,
        _page: PdfPage,
    ) {
    }

    fn will_end_displaying_overlay_view(
        &mut self,
        _view: PdfView,
        _overlay_view: PdfPageOverlayView,
        _page: PdfPage,
    ) {
    }
}

struct ProviderState {
    provider: Box<dyn PdfPageOverlayViewProvider>,
}

pub struct PdfPageOverlayViewProviderHandle {
    handle: ObjectHandle,
    _state: Box<ProviderState>,
}

impl PdfPageOverlayViewProviderHandle {
    pub fn new(provider: impl PdfPageOverlayViewProvider) -> Result<Self> {
        let mut state = Box::new(ProviderState {
            provider: Box::new(provider),
        });
        let context = ptr::addr_of_mut!(*state).cast::<c_void>();
        let mut out_provider = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_page_overlay_view_provider_new(
                context,
                Some(pdf_page_overlay_view_provider_overlay_trampoline),
                Some(pdf_page_overlay_view_provider_will_display_trampoline),
                Some(pdf_page_overlay_view_provider_will_end_displaying_trampoline),
                &mut out_provider,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self {
            handle: crate::util::required_handle(out_provider, "PDFPageOverlayViewProvider")?,
            _state: state,
        })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl fmt::Debug for PdfPageOverlayViewProviderHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PdfPageOverlayViewProviderHandle")
            .finish_non_exhaustive()
    }
}

unsafe fn provider_state(context: *mut c_void) -> Option<&'static mut ProviderState> {
    context.cast::<ProviderState>().as_mut()
}

unsafe fn retained_view(handle: *mut c_void) -> Option<PdfView> {
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfView::from_handle)
}

unsafe fn retained_page(handle: *mut c_void) -> Option<PdfPage> {
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfPage::from_handle)
}

unsafe fn retained_overlay_view(handle: *mut c_void) -> Option<PdfPageOverlayView> {
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfPageOverlayView::from_handle)
}

unsafe extern "C" fn pdf_page_overlay_view_provider_overlay_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    page_handle: *mut c_void,
) -> *mut c_void {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { provider_state(context) }) else {
            return ptr::null_mut();
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return ptr::null_mut();
        };
        let Some(page) = (unsafe { retained_page(page_handle) }) else {
            return ptr::null_mut();
        };
        state
            .provider
            .overlay_view_for_page(view, page)
            .map_or(ptr::null_mut(), PdfPageOverlayView::into_handle_ptr)
    }))
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn pdf_page_overlay_view_provider_will_display_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    overlay_view_handle: *mut c_void,
    page_handle: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { provider_state(context) }) else {
            return;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return;
        };
        let Some(overlay_view) = (unsafe { retained_overlay_view(overlay_view_handle) }) else {
            return;
        };
        let Some(page) = (unsafe { retained_page(page_handle) }) else {
            return;
        };
        state
            .provider
            .will_display_overlay_view(view, overlay_view, page);
    }));
}

unsafe extern "C" fn pdf_page_overlay_view_provider_will_end_displaying_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    overlay_view_handle: *mut c_void,
    page_handle: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { provider_state(context) }) else {
            return;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return;
        };
        let Some(overlay_view) = (unsafe { retained_overlay_view(overlay_view_handle) }) else {
            return;
        };
        let Some(page) = (unsafe { retained_page(page_handle) }) else {
            return;
        };
        state
            .provider
            .will_end_displaying_overlay_view(view, overlay_view, page);
    }));
}
