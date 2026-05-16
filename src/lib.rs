#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation
//!
//! Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit)
//! framework — PDF documents, pages, selections, outlines, and annotations on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::return_self_not_must_use,
    clippy::struct_excessive_bools
)]

mod annotation;
mod document;
mod error;
mod ffi;
mod handle;
mod outline;
mod page;
mod selection;
mod types;
mod util;

pub use annotation::PdfAnnotation;
pub use document::PdfDocument;
pub use error::{PdfKitError, Result};
pub use outline::PdfOutline;
pub use page::PdfPage;
pub use selection::PdfSelection;
pub use types::{
    DisplayBox, PdfAnnotationInfo, PdfDocumentAttributes, PdfDocumentInfo, PdfDocumentPermissions,
    PdfRect,
};

pub mod prelude {
    pub use crate::{
        DisplayBox, PdfAnnotation, PdfAnnotationInfo, PdfDocument, PdfDocumentAttributes,
        PdfDocumentInfo, PdfDocumentPermissions, PdfKitError, PdfOutline, PdfPage, PdfRect,
        PdfSelection, Result,
    };
}
