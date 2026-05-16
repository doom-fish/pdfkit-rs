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
mod action;
mod action_goto;
mod action_named;
mod action_remote_goto;
mod action_url;
mod annotation;
mod appearance_characteristics;
mod border;
mod destination;
mod document;
mod document_delegate;
mod error;
mod ffi;
mod handle;
mod notifications;
mod outline;
mod page;
mod selection;
mod thumbnail_view;
mod types;
mod util;
mod view;

pub use accessibility_node::PdfAccessibilityNode;
pub use action::{PdfAction, PdfActionLike};
pub use action_goto::PdfActionGoTo;
pub use action_named::PdfActionNamed;
pub use action_remote_goto::PdfActionRemoteGoTo;
pub use action_url::PdfActionUrl;
pub use annotation::PdfAnnotation;
pub use appearance_characteristics::PdfAppearanceCharacteristics;
pub use border::PdfBorder;
pub use destination::PdfDestination;
pub use document::PdfDocument;
pub use document_delegate::{PdfDocumentDelegate, PdfDocumentDelegateHandle};
pub use error::{PdfKitError, Result};
pub use notifications::{
    PdfDocumentNotification, PdfDocumentNotificationUserInfoKey, PdfThumbnailViewNotification,
    PdfViewNotification,
};
pub use outline::PdfOutline;
pub use page::PdfPage;
pub use selection::PdfSelection;
pub use thumbnail_view::PdfThumbnailView;
pub use types::{
    DisplayBox, PdfActionNamedName, PdfAnnotationInfo, PdfAppearanceCharacteristicsInfo,
    PdfBorderInfo, PdfBorderStyle, PdfColor, PdfDestinationInfo, PdfDisplayDirection,
    PdfDisplayMode, PdfDocumentAttributes, PdfDocumentInfo, PdfDocumentPermissions,
    PdfDocumentWriteOptions, PdfEdgeInsets, PdfInterpolationQuality, PdfLineStyle,
    PdfMarkupType, PdfPoint, PdfRect, PdfSize, PdfTextRange, PdfThumbnailViewInfo,
    PdfViewInfo, PdfWidgetControlType,
};
pub use view::PdfView;

pub mod prelude {
    pub use crate::{
        DisplayBox, PdfAccessibilityNode, PdfAction, PdfActionGoTo, PdfActionNamed,
        PdfActionNamedName, PdfActionRemoteGoTo, PdfActionUrl, PdfAnnotation,
        PdfAnnotationInfo, PdfAppearanceCharacteristics, PdfAppearanceCharacteristicsInfo,
        PdfBorder, PdfBorderInfo, PdfBorderStyle, PdfColor, PdfDestination,
        PdfDestinationInfo, PdfDisplayDirection, PdfDisplayMode, PdfDocument,
        PdfDocumentAttributes, PdfDocumentDelegate, PdfDocumentDelegateHandle,
        PdfDocumentInfo, PdfDocumentNotification, PdfDocumentNotificationUserInfoKey,
        PdfDocumentPermissions, PdfDocumentWriteOptions, PdfEdgeInsets,
        PdfInterpolationQuality, PdfKitError, PdfLineStyle, PdfMarkupType, PdfOutline,
        PdfPage, PdfPoint, PdfRect, PdfSelection, PdfSize, PdfTextRange,
        PdfThumbnailView, PdfThumbnailViewInfo, PdfThumbnailViewNotification, PdfView,
        PdfViewInfo, PdfViewNotification, PdfWidgetControlType, Result,
    };
}
