use crate::action_goto::PdfActionGoTo;
use crate::action_named::PdfActionNamed;
use crate::action_remote_goto::PdfActionRemoteGoTo;
use crate::action_url::PdfActionUrl;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::take_string;

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub trait PdfActionLike: sealed::Sealed {
    #[doc(hidden)]
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void;
}

#[derive(Debug, Clone)]
pub struct PdfAction {
    handle: ObjectHandle,
}

impl PdfAction {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    #[must_use]
    pub fn action_type(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_action_type_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn as_url(&self) -> Option<PdfActionUrl> {
        let ptr = unsafe { ffi::pdf_action_as_url(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionUrl::from_handle)
    }

    #[must_use]
    pub fn as_goto(&self) -> Option<PdfActionGoTo> {
        let ptr = unsafe { ffi::pdf_action_as_goto(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionGoTo::from_handle)
    }

    #[must_use]
    pub fn as_named(&self) -> Option<PdfActionNamed> {
        let ptr = unsafe { ffi::pdf_action_as_named(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionNamed::from_handle)
    }

    #[must_use]
    pub fn as_remote_goto(&self) -> Option<PdfActionRemoteGoTo> {
        let ptr = unsafe { ffi::pdf_action_as_remote_goto(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionRemoteGoTo::from_handle)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}

impl sealed::Sealed for PdfAction {}

impl PdfActionLike for PdfAction {
    fn as_action_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.as_handle_ptr()
    }
}
