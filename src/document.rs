use std::path::Path;
use std::ptr;

use crate::document_delegate::PdfDocumentDelegateHandle;
use crate::error::{PdfKitError, Result};
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::outline::PdfOutline;
use crate::page::PdfPage;
use crate::selection::PdfSelection;
use crate::types::{
    PdfDocumentAttributes, PdfDocumentInfo, PdfDocumentWriteOptions, PdfPoint,
    PdfSelectionGranularity,
};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct PdfDocument {
    handle: ObjectHandle,
}

impl PdfDocument {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_document_new(&mut out_document, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    pub fn from_url(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_new_with_url(path.as_ptr(), &mut out_document, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_new_with_data(
                bytes.as_ptr(),
                bytes.len(),
                &mut out_document,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    pub fn info(&self) -> Result<PdfDocumentInfo> {
        parse_json(
            unsafe { ffi::pdf_document_info_json(self.handle.as_ptr()) },
            "PDFDocument",
        )
    }

    pub fn attributes(&self) -> Result<PdfDocumentAttributes> {
        parse_json(
            unsafe { ffi::pdf_document_attributes_json(self.handle.as_ptr()) },
            "PDFDocument attributes",
        )
    }

    #[must_use]
    pub fn string(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_document_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn page_count(&self) -> usize {
        unsafe { ffi::pdf_document_page_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn page(&self, index: usize) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_document_page_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    #[must_use]
    pub fn pages(&self) -> Vec<PdfPage> {
        (0..self.page_count())
            .filter_map(|index| self.page(index))
            .collect()
    }

    #[must_use]
    pub fn page_index(&self, page: &PdfPage) -> Option<usize> {
        let index =
            unsafe { ffi::pdf_document_index_for_page(self.handle.as_ptr(), page.as_handle_ptr()) };
        (index != u64::MAX).then_some(index as usize)
    }

    #[must_use]
    pub fn outline_root(&self) -> Option<PdfOutline> {
        let ptr = unsafe { ffi::pdf_document_outline_root(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfOutline::from_handle)
    }

    pub fn set_outline_root(&self, outline: Option<&PdfOutline>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_set_outline_root(
                self.handle.as_ptr(),
                outline.map_or(ptr::null_mut(), PdfOutline::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn outline_item_for_selection(&self, selection: &PdfSelection) -> Option<PdfOutline> {
        let ptr = unsafe {
            ffi::pdf_document_outline_item_for_selection(
                self.handle.as_ptr(),
                selection.as_handle_ptr(),
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfOutline::from_handle)
    }

    #[must_use]
    pub fn selection_for_entire_document(&self) -> Option<PdfSelection> {
        let ptr = unsafe { ffi::pdf_document_selection_for_entire_document(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_from_page_points(
        &self,
        start_page: &PdfPage,
        start_point: PdfPoint,
        end_page: &PdfPage,
        end_point: PdfPoint,
    ) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_document_selection_from_pages_points(
                self.handle.as_ptr(),
                start_page.as_handle_ptr(),
                start_point.x,
                start_point.y,
                end_page.as_handle_ptr(),
                end_point.x,
                end_point.y,
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_from_page_points_with_granularity(
        &self,
        start_page: &PdfPage,
        start_point: PdfPoint,
        end_page: &PdfPage,
        end_point: PdfPoint,
        granularity: PdfSelectionGranularity,
    ) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_document_selection_from_pages_points_with_granularity(
                self.handle.as_ptr(),
                start_page.as_handle_ptr(),
                start_point.x,
                start_point.y,
                end_page.as_handle_ptr(),
                end_point.x,
                end_point.y,
                granularity.as_raw(),
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    #[must_use]
    pub fn selection_from_page_characters(
        &self,
        start_page: &PdfPage,
        start_character: usize,
        end_page: &PdfPage,
        end_character: usize,
    ) -> Option<PdfSelection> {
        let ptr = unsafe {
            ffi::pdf_document_selection_from_pages_characters(
                self.handle.as_ptr(),
                start_page.as_handle_ptr(),
                start_character as u64,
                end_page.as_handle_ptr(),
                end_character as u64,
            )
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    pub fn unlock(&self, password: &str) -> Result<bool> {
        let password = c_string(password)?;
        Ok(unsafe { ffi::pdf_document_unlock(self.handle.as_ptr(), password.as_ptr()) != 0 })
    }

    pub fn set_delegate(&self, delegate: Option<&PdfDocumentDelegateHandle>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_set_delegate(
                self.handle.as_ptr(),
                delegate.map_or(ptr::null_mut(), PdfDocumentDelegateHandle::as_handle_ptr),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn write_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_write_to_url(self.handle.as_ptr(), path.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    pub fn write_to_url_with_options(
        &self,
        path: impl AsRef<Path>,
        options: &PdfDocumentWriteOptions,
    ) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let options_json = serde_json::to_string(options).map_err(|error| {
            PdfKitError::new(
                ffi::status::FRAMEWORK,
                format!("failed to encode PDFDocument write options: {error}"),
            )
        })?;
        let options_json = c_string(&options_json)?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_write_to_url_with_options(
                self.handle.as_ptr(),
                path.as_ptr(),
                options_json.as_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn insert_page(&self, page: &PdfPage, index: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_insert_page(
                self.handle.as_ptr(),
                page.as_handle_ptr(),
                index as u64,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub fn remove_page(&self, index: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_remove_page_at(self.handle.as_ptr(), index as u64, &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    pub fn exchange_pages(&self, index_a: usize, index_b: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_exchange_pages(
                self.handle.as_ptr(),
                index_a as u64,
                index_b as u64,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
