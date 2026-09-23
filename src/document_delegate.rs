use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::sync::{Mutex, PoisonError};
use std::thread::{self, ThreadId};

use doom_fish_utils::callback_context::CallbackContext;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::notifications::PdfDocumentNotification;
use crate::selection::PdfSelection;

/// Mirrors the `PDFDocumentDelegate` callback surface.
pub trait PdfDocumentDelegate: Send + 'static {
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
    delegate: Mutex<Box<dyn PdfDocumentDelegate>>,
    caller: Mutex<Option<ThreadId>>,
}

type DelegateContext = CallbackContext<DelegateState>;

struct CallerReset<'a>(&'a Mutex<Option<ThreadId>>);

impl Drop for CallerReset<'_> {
    fn drop(&mut self) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }
}

impl DelegateState {
    fn with_delegate<R>(&self, f: impl FnOnce(&mut dyn PdfDocumentDelegate) -> R) -> Option<R> {
        let current = thread::current().id();
        if *self.caller.lock().unwrap_or_else(PoisonError::into_inner) == Some(current) {
            return None;
        }
        let mut delegate = self.delegate.lock().unwrap_or_else(PoisonError::into_inner);
        *self.caller.lock().unwrap_or_else(PoisonError::into_inner) = Some(current);
        let _reset = CallerReset(&self.caller);
        Some(f(delegate.as_mut()))
    }
}

/// Wraps `PDFDocumentDelegateHandle`.
pub struct PdfDocumentDelegateHandle {
    handle: ObjectHandle,
    context: DelegateContext,
}

impl PdfDocumentDelegateHandle {
    /// Registers a Rust implementation of `PDFDocumentDelegate`.
    pub fn new(delegate: impl PdfDocumentDelegate) -> Result<Self> {
        let context = DelegateContext::new(DelegateState {
            delegate: Mutex::new(Box::new(delegate)),
            caller: Mutex::new(None),
        });
        let mut out_delegate = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_delegate_new(
                context.as_ptr(),
                Some(pdf_document_delegate_notification_trampoline),
                Some(pdf_document_delegate_match_trampoline),
                Some(pdf_document_delegate_page_class_name_trampoline),
                Some(pdf_document_delegate_annotation_class_name_trampoline),
                DelegateContext::RETAIN,
                DelegateContext::RELEASE,
                &mut out_delegate,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        let handle = crate::util::required_handle(out_delegate, "PDFDocumentDelegate")?;
        Ok(Self { handle, context })
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for PdfDocumentDelegateHandle {
    fn drop(&mut self) {
        self.context.deactivate();
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
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, kept alive by the Swift delegate object. Panics are caught to
/// prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_notification_trampoline(
    context: *mut c_void,
    raw_notification: i32,
) {
    let _ = DelegateContext::with(context, "pdf_document_delegate_notification", |state| {
        let notification = PdfDocumentNotification::from_raw(raw_notification)?;
        state.with_delegate(|delegate| delegate.handle_notification(notification))
    });
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `selection_handle` must be a retained PDFSelection pointer
/// from Swift (or null); it is released even when the delegate is gone. Panics are caught to
/// prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_match_trampoline(
    context: *mut c_void,
    selection_handle: *mut c_void,
) {
    let Some(handle) = (unsafe { ObjectHandle::from_retained_ptr(selection_handle) }) else {
        return;
    };
    let selection = PdfSelection::from_handle(handle);
    let _ = DelegateContext::with(context, "pdf_document_delegate_match", move |state| {
        state.with_delegate(move |delegate| delegate.did_match_string(selection))
    });
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer. The returned pointer must be freed by the Swift caller. Panics
/// are caught to prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_page_class_name_trampoline(
    context: *mut c_void,
) -> *mut c_char {
    DelegateContext::with(context, "pdf_document_delegate_page_class_name", |state| {
        state.with_delegate(|delegate| duplicate_string(delegate.page_class_name()))
    })
    .flatten()
    .unwrap_or(ptr::null_mut())
}

/// # Safety
/// This is an extern "C" callback invoked by Swift. `context` must be null or the delegate's
/// `CallbackContext` pointer, and `annotation_type` must be either null or a valid C string
/// pointer. The returned pointer must be freed by the Swift caller. Panics are caught to
/// prevent unwinding across the FFI boundary.
unsafe extern "C" fn pdf_document_delegate_annotation_class_name_trampoline(
    context: *mut c_void,
    annotation_type: *const c_char,
) -> *mut c_char {
    if annotation_type.is_null() {
        return ptr::null_mut();
    }
    DelegateContext::with(context, "pdf_document_delegate_annotation_class_name", |state| {
        let annotation_type = unsafe { CStr::from_ptr(annotation_type) }
            .to_string_lossy()
            .into_owned();
        state.with_delegate(|delegate| {
            duplicate_string(delegate.annotation_class_name(&annotation_type))
        })
    })
    .flatten()
    .unwrap_or(ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;
    use std::time::Duration;

    use super::{
        pdf_document_delegate_notification_trampoline,
        pdf_document_delegate_page_class_name_trampoline, DelegateContext, DelegateState,
        PdfDocumentDelegate,
    };
    use crate::notifications::PdfDocumentNotification;

    fn context(delegate: impl PdfDocumentDelegate) -> DelegateContext {
        DelegateContext::new(DelegateState {
            delegate: Mutex::new(Box::new(delegate)),
            caller: Mutex::new(None),
        })
    }

    struct Reentrant {
        context: Arc<AtomicUsize>,
        notifications: Arc<Mutex<Vec<PdfDocumentNotification>>>,
    }

    impl PdfDocumentDelegate for Reentrant {
        fn handle_notification(&mut self, notification: PdfDocumentNotification) {
            self.notifications.lock().unwrap().push(notification);
            let context = self.context.load(Ordering::SeqCst) as *mut c_void;
            unsafe { pdf_document_delegate_notification_trampoline(context, 1) };
            self.notifications.lock().unwrap().push(notification);
        }

        fn page_class_name(&mut self) -> Option<String> {
            Some("PDFPage".to_owned())
        }
    }

    #[test]
    fn reentrant_callback_is_skipped_instead_of_aliasing_the_delegate() {
        let pointer = Arc::new(AtomicUsize::new(0));
        let notifications = Arc::new(Mutex::new(Vec::new()));
        let context = context(Reentrant {
            context: Arc::clone(&pointer),
            notifications: Arc::clone(&notifications),
        });
        pointer.store(context.as_ptr() as usize, Ordering::SeqCst);

        unsafe { pdf_document_delegate_notification_trampoline(context.as_ptr(), 0) };

        assert_eq!(
            *notifications.lock().unwrap(),
            [PdfDocumentNotification::DidUnlock, PdfDocumentNotification::DidUnlock]
        );

        let class_name =
            unsafe { pdf_document_delegate_page_class_name_trampoline(context.as_ptr()) };
        assert!(!class_name.is_null());
        unsafe { libc::free(class_name.cast()) };
    }

    struct Overlap {
        in_call: Arc<AtomicBool>,
        overlaps: Arc<AtomicUsize>,
        calls: Arc<AtomicUsize>,
    }

    impl PdfDocumentDelegate for Overlap {
        fn handle_notification(&mut self, _notification: PdfDocumentNotification) {
            if self.in_call.swap(true, Ordering::SeqCst) {
                self.overlaps.fetch_add(1, Ordering::SeqCst);
            }
            thread::sleep(Duration::from_millis(1));
            self.in_call.store(false, Ordering::SeqCst);
            self.calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn concurrent_callbacks_are_serialized() {
        const THREADS: usize = 4;
        const CALLS: usize = 25;
        let overlaps = Arc::new(AtomicUsize::new(0));
        let calls = Arc::new(AtomicUsize::new(0));
        let context = context(Overlap {
            in_call: Arc::new(AtomicBool::new(false)),
            overlaps: Arc::clone(&overlaps),
            calls: Arc::clone(&calls),
        });
        let barrier = Barrier::new(THREADS);

        thread::scope(|scope| {
            for _ in 0..THREADS {
                scope.spawn(|| {
                    let pointer = context.retained_ptr();
                    barrier.wait();
                    for _ in 0..CALLS {
                        unsafe { pdf_document_delegate_notification_trampoline(pointer, 2) };
                    }
                    unsafe { (DelegateContext::RELEASE)(pointer) };
                });
            }
        });

        assert_eq!(calls.load(Ordering::SeqCst), THREADS * CALLS);
        assert_eq!(overlaps.load(Ordering::SeqCst), 0);
    }

    struct Counting(Arc<AtomicUsize>);

    impl PdfDocumentDelegate for Counting {
        fn handle_notification(&mut self, _notification: PdfDocumentNotification) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn callbacks_after_the_handle_is_dropped_do_not_reach_the_delegate() {
        let calls = Arc::new(AtomicUsize::new(0));
        let context = context(Counting(Arc::clone(&calls)));
        let swift_reference = context.retained_ptr();

        unsafe { pdf_document_delegate_notification_trampoline(swift_reference, 5) };
        context.deactivate();
        unsafe { pdf_document_delegate_notification_trampoline(swift_reference, 5) };
        drop(context);
        unsafe { pdf_document_delegate_notification_trampoline(swift_reference, 5) };

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(Arc::strong_count(&calls), 2);
        unsafe { (DelegateContext::RELEASE)(swift_reference) };
        assert_eq!(Arc::strong_count(&calls), 1);
    }

    #[test]
    fn unknown_notifications_and_null_contexts_are_ignored() {
        let calls = Arc::new(AtomicUsize::new(0));
        let context = context(Counting(Arc::clone(&calls)));

        unsafe {
            pdf_document_delegate_notification_trampoline(context.as_ptr(), 99);
            pdf_document_delegate_notification_trampoline(std::ptr::null_mut(), 0);
        }

        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}
