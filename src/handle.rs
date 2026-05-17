use std::ptr::NonNull;

use crate::ffi;

#[derive(Debug)]
pub(crate) struct ObjectHandle(NonNull<core::ffi::c_void>);

impl ObjectHandle {
    /// Creates an ObjectHandle from a retained pointer.
    ///
    /// # Safety
    /// `ptr` must be either null or a valid, retained pointer to a PDFKit object. The pointer
    /// must have been retained by the Swift bridge, and ownership is transferred to this handle
    /// which will release it when dropped.
    pub(crate) unsafe fn from_retained_ptr(ptr: *mut core::ffi::c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.0.as_ptr()
    }
}

impl Clone for ObjectHandle {
    fn clone(&self) -> Self {
        // SAFETY: as_ptr returns a valid pointer to a PDFKit object; pdf_object_retain
        // is guaranteed to return a valid retained pointer or null (checked below)
        let retained = unsafe { ffi::pdf_object_retain(self.as_ptr()) };
        // SAFETY: retained pointer is valid or null (both handled by from_retained_ptr)
        unsafe { Self::from_retained_ptr(retained) }.expect("PDFKit retain returned null")
    }
}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        // SAFETY: as_ptr returns a valid pointer to a PDFKit object that was retained
        // when created; pdf_object_release is safe to call exactly once when dropping
        unsafe { ffi::pdf_object_release(self.as_ptr()) };
    }
}
