use std::ptr;

use crate::action_goto::PdfActionGoTo;
use crate::action_url::PdfActionUrl;
use crate::destination::PdfDestination;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{option_c_string, take_string};

#[derive(Debug, Clone)]
pub struct PdfOutline {
    handle: ObjectHandle,
}

impl PdfOutline {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_outline = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_outline_new(&mut out_outline, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_outline,
            "PDFOutline",
        )?))
    }

    #[must_use]
    pub fn label(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_outline_label_string(self.handle.as_ptr()) })
    }

    pub fn set_label(&self, value: Option<&str>) -> Result<()> {
        let value = option_c_string(value)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_set_label(
                self.handle.as_ptr(),
                value.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn child_count(&self) -> usize {
        unsafe { ffi::pdf_outline_child_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn child(&self, index: usize) -> Option<Self> {
        let ptr = unsafe { ffi::pdf_outline_child_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        (0..self.child_count())
            .filter_map(|index| self.child(index))
            .collect()
    }

    pub fn insert_child(&self, child: &Self, index: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_insert_child(
                self.handle.as_ptr(),
                child.handle.as_ptr(),
                index as u64,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn remove_from_parent(&self) {
        unsafe { ffi::pdf_outline_remove_from_parent(self.handle.as_ptr()) };
    }

    #[must_use]
    pub fn index(&self) -> usize {
        unsafe { ffi::pdf_outline_index(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { ffi::pdf_outline_is_open(self.handle.as_ptr()) != 0 }
    }

    pub fn set_open(&self, value: bool) {
        unsafe { ffi::pdf_outline_set_open(self.handle.as_ptr(), i32::from(value)) };
    }

    #[must_use]
    pub fn destination(&self) -> Option<PdfDestination> {
        let ptr = unsafe { ffi::pdf_outline_destination(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfDestination::from_handle)
    }

    pub fn set_destination(&self, destination: Option<&PdfDestination>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_set_destination(
                self.handle.as_ptr(),
                destination.map_or(ptr::null_mut(), PdfDestination::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_url(&self) -> Option<PdfActionUrl> {
        let ptr = unsafe { ffi::pdf_outline_action_url(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionUrl::from_handle)
    }

    pub fn set_action_url(&self, action: Option<&PdfActionUrl>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_set_action_url(
                self.handle.as_ptr(),
                action.map_or(ptr::null_mut(), PdfActionUrl::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn action_goto(&self) -> Option<PdfActionGoTo> {
        let ptr = unsafe { ffi::pdf_outline_action_goto(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionGoTo::from_handle)
    }

    pub fn set_action_goto(&self, action: Option<&PdfActionGoTo>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_set_action_goto(
                self.handle.as_ptr(),
                action.map_or(ptr::null_mut(), PdfActionGoTo::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        let ptr = unsafe { ffi::pdf_outline_parent(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
