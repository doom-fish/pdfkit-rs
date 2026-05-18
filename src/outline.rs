use std::ptr;

use crate::action::{PdfAction, PdfActionLike};
use crate::action_goto::PdfActionGoTo;
use crate::action_url::PdfActionUrl;
use crate::destination::PdfDestination;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{option_c_string, take_string};

/// Wraps `PDFOutline`.
#[derive(Debug, Clone)]
pub struct PdfOutline {
    handle: ObjectHandle,
}

impl PdfOutline {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps `PDFOutline()`.
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

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_outline_label_string(self.handle.as_ptr()) })
    }

    /// Wraps the corresponding `PDFOutline` API.
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

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn child_count(&self) -> usize {
        unsafe { ffi::pdf_outline_child_count(self.handle.as_ptr()) as usize }
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn child(&self, index: usize) -> Option<Self> {
        let ptr = unsafe { ffi::pdf_outline_child_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        (0..self.child_count())
            .filter_map(|index| self.child(index))
            .collect()
    }

    /// Wraps the corresponding `PDFOutline` API.
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

    /// Wraps the corresponding `PDFOutline` API.
    pub fn remove_from_parent(&self) {
        unsafe { ffi::pdf_outline_remove_from_parent(self.handle.as_ptr()) };
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn index(&self) -> usize {
        unsafe { ffi::pdf_outline_index(self.handle.as_ptr()) as usize }
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { ffi::pdf_outline_is_open(self.handle.as_ptr()) != 0 }
    }

    /// Wraps the corresponding `PDFOutline` API.
    pub fn set_open(&self, value: bool) {
        unsafe { ffi::pdf_outline_set_open(self.handle.as_ptr(), i32::from(value)) };
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn destination(&self) -> Option<PdfDestination> {
        let ptr = unsafe { ffi::pdf_outline_destination(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfDestination::from_handle)
    }

    /// Wraps the corresponding `PDFOutline` API.
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

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn action(&self) -> Option<PdfAction> {
        let ptr = unsafe { ffi::pdf_outline_action(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfAction::from_handle)
    }

    /// Wraps the corresponding `PDFOutline` API.
    pub fn set_action<A: PdfActionLike>(&self, action: Option<&A>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_outline_set_action(
                self.handle.as_ptr(),
                action.map_or(ptr::null_mut(), PdfActionLike::as_action_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFOutline` API.
    pub fn clear_action(&self) -> Result<()> {
        self.set_action::<PdfAction>(None)
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn action_url(&self) -> Option<PdfActionUrl> {
        let ptr = unsafe { ffi::pdf_outline_action_url(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionUrl::from_handle)
    }

    /// Wraps the corresponding `PDFOutline` API.
    pub fn set_action_url(&self, action: Option<&PdfActionUrl>) -> Result<()> {
        self.set_action(action)
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn action_goto(&self) -> Option<PdfActionGoTo> {
        let ptr = unsafe { ffi::pdf_outline_action_goto(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfActionGoTo::from_handle)
    }

    /// Wraps the corresponding `PDFOutline` API.
    pub fn set_action_goto(&self, action: Option<&PdfActionGoTo>) -> Result<()> {
        self.set_action(action)
    }

    /// Wraps the corresponding `PDFOutline` API.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        let ptr = unsafe { ffi::pdf_outline_parent(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
