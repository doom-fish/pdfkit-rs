use std::ptr;

use crate::destination::PdfDestination;
use crate::document::PdfDocument;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::selection::PdfSelection;
use crate::types::{DisplayBox, PdfDisplayDirection, PdfDisplayMode, PdfSize, PdfViewInfo};
use crate::util::parse_json;

#[derive(Debug, Clone)]
pub struct PdfView {
    handle: ObjectHandle,
}

impl PdfView {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(size: PdfSize) -> Result<Self> {
        let mut out_view = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_view_new(size.width, size.height, &mut out_view, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_view,
            "PDFView",
        )?))
    }

    pub fn info(&self) -> Result<PdfViewInfo> {
        parse_json(unsafe { ffi::pdf_view_info_json(self.handle.as_ptr()) }, "PDFView")
    }

    #[must_use]
    pub fn document(&self) -> Option<PdfDocument> {
        let ptr = unsafe { ffi::pdf_view_document(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfDocument::from_handle)
    }

    pub fn set_document(&self, document: Option<&PdfDocument>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_set_document(
                self.handle.as_ptr(),
                document.map_or(ptr::null_mut(), PdfDocument::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn current_page(&self) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_view_current_page(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    #[must_use]
    pub fn current_destination(&self) -> Option<PdfDestination> {
        let ptr = unsafe { ffi::pdf_view_current_destination(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfDestination::from_handle)
    }

    #[must_use]
    pub fn current_selection(&self) -> Option<PdfSelection> {
        let ptr = unsafe { ffi::pdf_view_current_selection(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    pub fn set_current_selection(&self, selection: Option<&PdfSelection>, animate: bool) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_set_current_selection(
                self.handle.as_ptr(),
                selection.map_or(ptr::null_mut(), PdfSelection::as_handle_ptr),
                i32::from(animate),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn clear_selection(&self) {
        unsafe { ffi::pdf_view_clear_selection(self.handle.as_ptr()) };
    }

    pub fn go_to_page(&self, page: &PdfPage) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_view_go_to_page(self.handle.as_ptr(), page.as_handle_ptr(), &mut out_error) };
        crate::util::status_result(status, out_error)
    }

    pub fn go_to_destination(&self, destination: &PdfDestination) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_go_to_destination(
                self.handle.as_ptr(),
                destination.as_handle_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn go_to_selection(&self, selection: &PdfSelection) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_go_to_selection(
                self.handle.as_ptr(),
                selection.as_handle_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn set_display_mode(&self, mode: PdfDisplayMode) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_view_set_display_mode(self.handle.as_ptr(), mode as i32, &mut out_error) };
        crate::util::status_result(status, out_error)
    }

    pub fn set_display_direction(&self, direction: PdfDisplayDirection) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_set_display_direction(self.handle.as_ptr(), direction as i32, &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    pub fn set_display_box(&self, display_box: DisplayBox) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_view_set_display_box(self.handle.as_ptr(), display_box.as_raw(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    pub fn set_auto_scales(&self, value: bool) {
        unsafe { ffi::pdf_view_set_auto_scales(self.handle.as_ptr(), i32::from(value)) };
    }

    pub fn set_scale_factor(&self, value: f64) {
        unsafe { ffi::pdf_view_set_scale_factor(self.handle.as_ptr(), value) };
    }

    pub fn set_min_scale_factor(&self, value: f64) {
        unsafe { ffi::pdf_view_set_min_scale_factor(self.handle.as_ptr(), value) };
    }

    pub fn set_max_scale_factor(&self, value: f64) {
        unsafe { ffi::pdf_view_set_max_scale_factor(self.handle.as_ptr(), value) };
    }

    pub fn layout_document_view(&self) {
        unsafe { ffi::pdf_view_layout_document_view(self.handle.as_ptr()) };
    }

    #[must_use]
    pub fn visible_page_count(&self) -> usize {
        unsafe { ffi::pdf_view_visible_page_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn visible_page(&self, index: usize) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_view_visible_page_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    #[must_use]
    pub fn visible_pages(&self) -> Vec<PdfPage> {
        (0..self.visible_page_count())
            .filter_map(|index| self.visible_page(index))
            .collect()
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
