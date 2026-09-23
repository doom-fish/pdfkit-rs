use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

use zeroize::Zeroizing;

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
use crate::util::{parse_json, path_to_c_string, required_handle, secret_c_string, take_string};

/// Wraps `PDFDocument`.
#[derive(Debug, Clone)]
pub struct PdfDocument {
    handle: ObjectHandle,
}

impl PdfDocument {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps `PDFDocument()`.
    pub fn new() -> Result<Self> {
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::pdf_document_new(&raw mut out_document, &raw mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    /// Wraps `PDFDocument(url:)`.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_new_with_url(path.as_ptr(), &raw mut out_document, &raw mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    /// Wraps `PDFDocument(data:)`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut out_document = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_new_with_data(
                bytes.as_ptr(),
                bytes.len(),
                &raw mut out_document,
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_document,
            "PDFDocument",
        )?))
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn info(&self) -> Result<PdfDocumentInfo> {
        parse_json(
            unsafe { ffi::pdf_document_info_json(self.handle.as_ptr()) },
            "PDFDocument",
        )
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn attributes(&self) -> Result<PdfDocumentAttributes> {
        parse_json(
            unsafe { ffi::pdf_document_attributes_json(self.handle.as_ptr()) },
            "PDFDocument attributes",
        )
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn string(&self) -> Option<String> {
        take_string(unsafe { ffi::pdf_document_string(self.handle.as_ptr()) })
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn page_count(&self) -> usize {
        unsafe { ffi::pdf_document_page_count(self.handle.as_ptr()) as usize }
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn page(&self, index: usize) -> Option<PdfPage> {
        let ptr = unsafe { ffi::pdf_document_page_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfPage::from_handle)
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn pages(&self) -> Vec<PdfPage> {
        (0..self.page_count())
            .filter_map(|index| self.page(index))
            .collect()
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn page_index(&self, page: &PdfPage) -> Option<usize> {
        let index =
            unsafe { ffi::pdf_document_index_for_page(self.handle.as_ptr(), page.as_handle_ptr()) };
        (index != u64::MAX).then_some(index as usize)
    }

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn outline_root(&self) -> Option<PdfOutline> {
        let ptr = unsafe { ffi::pdf_document_outline_root(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfOutline::from_handle)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn set_outline_root(&self, outline: Option<&PdfOutline>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_set_outline_root(
                self.handle.as_ptr(),
                outline.map_or(ptr::null_mut(), PdfOutline::as_handle_ptr),
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
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

    /// Wraps the corresponding `PDFDocument` API.
    #[must_use]
    pub fn selection_for_entire_document(&self) -> Option<PdfSelection> {
        let ptr = unsafe { ffi::pdf_document_selection_for_entire_document(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PdfSelection::from_handle)
    }

    /// Wraps the corresponding `PDFDocument` API.
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

    /// Wraps the corresponding `PDFDocument` API.
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

    /// Wraps `PDFDocument.selection(from:atCharacterIndex:to:atCharacterIndex:)`. The character
    /// indexes count UTF-16 code units, like [`PdfPage::number_of_characters`].
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

    /// Wraps the corresponding `PDFDocument` API.
    pub fn unlock(&self, password: &str) -> Result<bool> {
        let password = secret_c_string(password)?;
        Ok(unsafe {
            ffi::pdf_document_unlock(self.handle.as_ptr(), password.as_ptr().cast::<c_char>()) != 0
        })
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn set_delegate(&self, delegate: Option<&PdfDocumentDelegateHandle>) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_set_delegate(
                self.handle.as_ptr(),
                delegate.map_or(ptr::null_mut(), PdfDocumentDelegateHandle::as_handle_ptr),
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn write_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_write_to_url(self.handle.as_ptr(), path.as_ptr(), &raw mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn write_to_url_with_options(
        &self,
        path: impl AsRef<Path>,
        options: &PdfDocumentWriteOptions,
    ) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let secret_len = options.owner_password.as_ref().map_or(0, |value| value.len())
            + options.user_password.as_ref().map_or(0, |value| value.len());
        let mut options_json = Zeroizing::new(Vec::new());
        secret_len
            .checked_mul(6)
            .and_then(|len| len.checked_add(512))
            .and_then(|capacity| options_json.try_reserve_exact(capacity).ok())
            .ok_or_else(|| {
                PdfKitError::new(
                    ffi::status::INVALID_ARGUMENT,
                    "PDFDocument write options are too large",
                )
            })?;
        serde_json::to_writer(&mut *options_json, options).map_err(|error| {
            PdfKitError::new(
                ffi::status::FRAMEWORK,
                format!("failed to encode PDFDocument write options: {error}"),
            )
        })?;
        options_json.push(0);
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_write_to_url_with_options(
                self.handle.as_ptr(),
                path.as_ptr(),
                options_json.as_ptr().cast::<c_char>(),
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn insert_page(&self, page: &PdfPage, index: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_insert_page(
                self.handle.as_ptr(),
                page.as_handle_ptr(),
                index as u64,
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn remove_page(&self, index: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_remove_page_at(self.handle.as_ptr(), index as u64, &raw mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    /// Wraps the corresponding `PDFDocument` API.
    pub fn exchange_pages(&self, index_a: usize, index_b: usize) -> Result<()> {
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_exchange_pages(
                self.handle.as_ptr(),
                index_a as u64,
                index_b as u64,
                &raw mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }

    pub(crate) fn as_handle_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }
}
