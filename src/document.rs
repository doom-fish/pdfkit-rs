use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::outline::PdfOutline;
use crate::page::PdfPage;
use crate::types::{PdfDocumentAttributes, PdfDocumentInfo};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct PdfDocument {
    handle: ObjectHandle,
}

impl PdfDocument {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
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
    pub fn outline_root(&self) -> Option<PdfOutline> {
        let ptr = unsafe { ffi::pdf_document_outline_root(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfOutline::from_handle)
    }

    pub fn unlock(&self, password: &str) -> Result<bool> {
        let password = c_string(password)?;
        Ok(unsafe { ffi::pdf_document_unlock(self.handle.as_ptr(), password.as_ptr()) != 0 })
    }

    pub fn write_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_write_to_url(self.handle.as_ptr(), path.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }
}
