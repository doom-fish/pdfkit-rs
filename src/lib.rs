#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation
//!
//! Safe Rust bindings for Apple's [PDFKit](https://developer.apple.com/documentation/pdfkit)
//! framework on macOS.

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

mod accessibility_node;
mod action_goto;
mod action_url;
mod annotation;
mod appearance_characteristics;
mod border;
mod destination;
mod document;
mod error;
mod ffi;
mod handle;
mod outline;
mod page;
mod selection;
mod thumbnail_view;
mod types;
mod util;
mod view;

pub use accessibility_node::PdfAccessibilityNode;
pub use action_goto::PdfActionGoTo;
pub use action_url::PdfActionUrl;
pub use annotation::PdfAnnotation;
pub use appearance_characteristics::PdfAppearanceCharacteristics;
pub use border::PdfBorder;
pub use destination::PdfDestination;
pub use document::PdfDocument;
pub use error::{PdfKitError, Result};
pub use outline::PdfOutline;
pub use page::PdfPage;
pub use selection::PdfSelection;
pub use thumbnail_view::PdfThumbnailView;
pub use types::{
    DisplayBox, PdfAnnotationInfo, PdfAppearanceCharacteristicsInfo, PdfBorderInfo,
    PdfBorderStyle, PdfColor, PdfDestinationInfo, PdfDisplayDirection, PdfDisplayMode,
    PdfDocumentAttributes, PdfDocumentInfo, PdfDocumentPermissions, PdfEdgeInsets,
    PdfInterpolationQuality, PdfPoint, PdfRect, PdfSize, PdfTextRange, PdfThumbnailViewInfo,
    PdfViewInfo, PdfWidgetControlType,
};
pub use view::PdfView;

pub mod prelude {
    pub use crate::{
        DisplayBox, PdfAccessibilityNode, PdfActionGoTo, PdfActionUrl, PdfAnnotation,
        PdfAnnotationInfo, PdfAppearanceCharacteristics, PdfAppearanceCharacteristicsInfo,
        PdfBorder, PdfBorderInfo, PdfBorderStyle, PdfColor, PdfDestination,
        PdfDestinationInfo, PdfDisplayDirection, PdfDisplayMode, PdfDocument,
        PdfDocumentAttributes, PdfDocumentInfo, PdfDocumentPermissions, PdfEdgeInsets,
        PdfInterpolationQuality, PdfKitError, PdfOutline, PdfPage, PdfPoint, PdfRect,
        PdfSelection, PdfSize, PdfTextRange, PdfThumbnailView, PdfThumbnailViewInfo,
        PdfView, PdfViewInfo, PdfWidgetControlType, Result,
    };
}
