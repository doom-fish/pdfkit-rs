use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::action_remote_goto::PdfActionRemoteGoTo;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::view::PdfView;

pub trait PdfViewDelegate: 'static {
    fn handle_link_click(&mut self, _view: PdfView, _url: &str) -> bool {
        false
    }

    fn will_change_scale_factor(&mut self, _view: PdfView, scale_factor: f64) -> f64 {
        scale_factor.clamp(0.1, 10.0)
    }

    fn print_job_title(&mut self, _view: PdfView) -> Option<String> {
        None
    }

    fn perform_print(&mut self, _view: PdfView) -> bool {
        false
    }

    fn perform_find(&mut self, _view: PdfView) -> bool {
        false
    }

    fn perform_go_to_page(&mut self, _view: PdfView) -> bool {
        false
    }

    fn open_pdf_for_remote_goto_action(
        &mut self,
        _view: PdfView,
        _action: PdfActionRemoteGoTo,
    ) -> bool {
        false
    }
}

struct DelegateState {
    delegate: Box<dyn PdfViewDelegate>,
}

pub struct PdfViewDelegateHandle {
    handle: ObjectHandle,
    _state: Box<DelegateState>,
}

impl PdfViewDelegateHandle {
    pub fn new(delegate: impl PdfViewDelegate) -> Result<Self> {
        let mut state = Box::new(DelegateState {
            delegate: Box::new(delegate),
        });
        let context = ptr::addr_of_mut!(*state).cast::<c_void>();
        let mut out_delegate = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_delegate_new(
                context,
                Some(pdf_view_delegate_link_click_trampoline),
                Some(pdf_view_delegate_scale_factor_trampoline),
                Some(pdf_view_delegate_print_job_title_trampoline),
                Some(pdf_view_delegate_perform_print_trampoline),
                Some(pdf_view_delegate_perform_find_trampoline),
                Some(pdf_view_delegate_perform_go_to_page_trampoline),
                Some(pdf_view_delegate_remote_goto_trampoline),
                &mut out_delegate,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self {
            handle: crate::util::required_handle(out_delegate, "PDFViewDelegate")?,
            _state: state,
        })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
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

unsafe fn delegate_state(context: *mut c_void) -> Option<&'static mut DelegateState> {
    context.cast::<DelegateState>().as_mut()
}

unsafe fn retained_view(handle: *mut c_void) -> Option<PdfView> {
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfView::from_handle)
}

unsafe fn retained_remote_goto_action(handle: *mut c_void) -> Option<PdfActionRemoteGoTo> {
    unsafe { ObjectHandle::from_retained_ptr(handle) }.map(PdfActionRemoteGoTo::from_handle)
}

unsafe extern "C" fn pdf_view_delegate_link_click_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    url: *const c_char,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return 0;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return 0;
        };
        let Some(url) =
            (!url.is_null()).then(|| unsafe { CStr::from_ptr(url).to_string_lossy().into_owned() })
        else {
            return 0;
        };
        i32::from(state.delegate.handle_link_click(view, &url))
    }))
    .unwrap_or(0)
}

unsafe extern "C" fn pdf_view_delegate_scale_factor_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    scale_factor: f64,
) -> f64 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return scale_factor.clamp(0.1, 10.0);
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return scale_factor.clamp(0.1, 10.0);
        };
        state.delegate.will_change_scale_factor(view, scale_factor)
    }))
    .unwrap_or_else(|_| scale_factor.clamp(0.1, 10.0))
}

unsafe extern "C" fn pdf_view_delegate_print_job_title_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return ptr::null_mut();
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return ptr::null_mut();
        };
        duplicate_string(state.delegate.print_job_title(view))
    }))
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn pdf_view_delegate_perform_print_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return 0;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return 0;
        };
        i32::from(state.delegate.perform_print(view))
    }))
    .unwrap_or(0)
}

unsafe extern "C" fn pdf_view_delegate_perform_find_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return 0;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return 0;
        };
        i32::from(state.delegate.perform_find(view))
    }))
    .unwrap_or(0)
}

unsafe extern "C" fn pdf_view_delegate_perform_go_to_page_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return 0;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return 0;
        };
        i32::from(state.delegate.perform_go_to_page(view))
    }))
    .unwrap_or(0)
}

unsafe extern "C" fn pdf_view_delegate_remote_goto_trampoline(
    context: *mut c_void,
    view_handle: *mut c_void,
    action_handle: *mut c_void,
) -> i32 {
    catch_unwind(AssertUnwindSafe(|| {
        let Some(state) = (unsafe { delegate_state(context) }) else {
            return 0;
        };
        let Some(view) = (unsafe { retained_view(view_handle) }) else {
            return 0;
        };
        let Some(action) = (unsafe { retained_remote_goto_action(action_handle) }) else {
            return 0;
        };
        i32::from(state.delegate.open_pdf_for_remote_goto_action(view, action))
    }))
    .unwrap_or(0)
}
