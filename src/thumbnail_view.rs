use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::page::PdfPage;
use crate::types::{PdfSize, PdfThumbnailViewInfo};
use crate::util::parse_json;
use crate::view::PdfView;

#[derive(Debug, Clone)]
pub struct PdfThumbnailView {
    handle: ObjectHandle,
}

impl PdfThumbnailView {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(size: PdfSize) -> Result<Self> {
        let mut out_view = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_thumbnail_view_new(size.width, size.height, &mut out_view, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_view,
            "PDFThumbnailView",
        )?))
    }

    pub fn info(&self) -> Result<PdfThumbnailViewInfo> {
        parse_json(
            unsafe { ffi::pdf_thumbnail_view_info_json(self.handle.as_ptr()) },
            "PDFThumbnailView",
        )
    }

    #[must_use]
    pub fn pdf_view(&self) -> Option<PdfView> {
        let ptr = unsafe { ffi::pdf_thumbnail_view_pdf_view(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfView::from_handle)
    }

    pub fn set_pdf_view(&self, pdf_view: Option<&PdfView>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_thumbnail_view_set_pdf_view(
                self.handle.as_ptr(),
                pdf_view.map_or(ptr::null_mut(), PdfView::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn set_thumbnail_size(&self, size: PdfSize) {
        unsafe { ffi::pdf_thumbnail_view_set_thumbnail_size(self.handle.as_ptr(), size.width, size.height) };
    }

    pub fn set_maximum_number_of_columns(&self, value: usize) {
        unsafe { ffi::pdf_thumbnail_view_set_maximum_number_of_columns(self.handle.as_ptr(), value as u64) };
    }

    pub fn set_allows_dragging(&self, value: bool) {
        unsafe { ffi::pdf_thumbnail_view_set_allows_dragging(self.handle.as_ptr(), i32::from(value)) };
    }

    pub fn set_allows_multiple_selection(&self, value: bool) {
        unsafe {
            ffi::pdf_thumbnail_view_set_allows_multiple_selection(self.handle.as_ptr(), i32::from(value));
        };
    }

    #[must_use]
    pub fn selected_page_count(&self) -> usize {
        unsafe { ffi::pdf_thumbnail_view_selected_page_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn selected_page(&self, index: usize) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_thumbnail_view_selected_page_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    #[must_use]
    pub fn selected_pages(&self) -> Vec<PdfPage> {
        (0..self.selected_page_count())
            .filter_map(|index| self.selected_page(index))
            .collect()
    }
}
