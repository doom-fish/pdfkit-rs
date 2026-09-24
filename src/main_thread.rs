use std::cell::RefCell;
use std::mem::ManuallyDrop;

use crate::error::{PdfKitError, Result};
use crate::ffi;

fn is_main_thread() -> bool {
    unsafe { libc::pthread_main_np() != 0 }
}

pub(crate) fn require_main_thread(what: &str) -> Result<()> {
    if is_main_thread() {
        Ok(())
    } else {
        Err(PdfKitError::new(
            ffi::status::WRONG_THREAD,
            format!("{what} must be created and used on the main thread"),
        ))
    }
}

pub(crate) struct MainThreadCell<T> {
    value: ManuallyDrop<RefCell<T>>,
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T> Send for MainThreadCell<T> {}

unsafe impl<T> Sync for MainThreadCell<T> {}

impl<T> MainThreadCell<T> {
    pub(crate) fn new(value: T, what: &str) -> Result<Self> {
        require_main_thread(what)?;
        Ok(Self {
            value: ManuallyDrop::new(RefCell::new(value)),
        })
    }

    pub(crate) fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        if !is_main_thread() {
            return None;
        }
        let mut value = self.value.try_borrow_mut().ok()?;
        Some(f(&mut value))
    }
}

impl<T> Drop for MainThreadCell<T> {
    fn drop(&mut self) {
        if is_main_thread() {
            unsafe { ManuallyDrop::drop(&mut self.value) };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    use super::{require_main_thread, MainThreadCell};
    use crate::ffi;

    struct Counted(Arc<AtomicUsize>);

    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn main_thread_requirements_fail_off_the_main_thread() {
        let drops = Arc::new(AtomicUsize::new(0));
        let value = Counted(Arc::clone(&drops));
        let (required, created) = thread::spawn(move || {
            (
                require_main_thread("PDFView").map_err(|error| error.code()),
                MainThreadCell::new(value, "delegate")
                    .map(drop)
                    .map_err(|error| error.code()),
            )
        })
        .join()
        .unwrap();

        assert_eq!(required, Err(ffi::status::WRONG_THREAD));
        assert_eq!(created, Err(ffi::status::WRONG_THREAD));
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
