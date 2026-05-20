//! Executor-agnostic async wrappers for PDFKit document finding.
//!
//! Enabled with the `async` Cargo feature.
//!
//! [`PdfDocumentFindStream`] runs `PDFDocument.findString(_:withOptions:)` on a
//! worker thread, converts every match into an owned Rust snapshot, and emits a
//! bounded async stream backed by [`doom_fish_utils::stream::BoundedAsyncStream`].
//!
//! The stream emits synthetic `DidBeginFind` / `DidEndFind` notifications around
//! the match sequence. Dropping it waits for the worker thread to finish and then
//! closes the stream.
//!
//! # Example
//!
//! ```no_run
//! use pdfkit::async_api::{
//!     PdfDocumentFindEvent, PdfDocumentFindOptions, PdfDocumentFindStream,
//! };
//! use pdfkit::PdfDocument;
//!
//! # async fn run() -> pdfkit::Result<()> {
//! let document = PdfDocument::from_url("examples/assets/hello.pdf")?;
//! let stream = PdfDocumentFindStream::find_string(
//!     &document,
//!     "Hello",
//!     PdfDocumentFindOptions::NONE,
//!     8,
//! )?;
//!
//! while let Some(event) = stream.next().await {
//!     match event {
//!         PdfDocumentFindEvent::Notification(notification) => {
//!             println!("notification={}", notification.name());
//!         }
//!         PdfDocumentFindEvent::Match(found) => {
//!             println!("match={:?} pages={:?}", found.text, found.pages);
//!         }
//!         PdfDocumentFindEvent::Failed(error) => {
//!             eprintln!("search failed: {error}");
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]

use core::ffi::c_void;
use core::fmt;
use std::ops::BitOr;
use std::ptr;
use std::thread::{self, JoinHandle};

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};
use serde::Deserialize;

use crate::error::{PdfKitError, Result};
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util;
use crate::{PdfDocument, PdfDocumentNotification, PdfTextRange};

/// `PDFDocument.findString(_:withOptions:)` comparison options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PdfDocumentFindOptions(u64);

impl PdfDocumentFindOptions {
    /// Default PDFKit string-find behaviour.
    pub const NONE: Self = Self(0);
    /// Case-insensitive matching.
    pub const CASE_INSENSITIVE: Self = Self(1);
    /// Literal matching without locale-aware folding.
    pub const LITERAL: Self = Self(1 << 1);
    /// Search backwards from the end of the document.
    pub const BACKWARDS: Self = Self(1 << 2);

    /// Return the raw `NSString.CompareOptions` bit pattern forwarded to PDFKit.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl BitOr for PdfDocumentFindOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// One page of a match snapshot emitted by [`PdfDocumentFindStream`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PdfDocumentFindPageMatch {
    /// Zero-based page index in the searched document.
    pub page_index: usize,
    /// Text ranges within that page that belong to the match.
    pub ranges: Vec<PdfTextRange>,
}

/// Owned snapshot of one `PDFDocument` string match.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PdfDocumentFindMatch {
    /// Plain-text representation of the match, if PDFKit can provide one.
    pub text: Option<String>,
    /// Per-page ranges that make up the match.
    pub pages: Vec<PdfDocumentFindPageMatch>,
}

/// Events emitted while a PDFKit string search is running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfDocumentFindEvent {
    /// Synthetic lifecycle notifications emitted by the worker thread.
    Notification(PdfDocumentNotification),
    /// One owned match snapshot.
    Match(PdfDocumentFindMatch),
    /// Search failure reported by the worker thread.
    Failed(PdfKitError),
}

struct SearchThreadHandle {
    join: Option<JoinHandle<()>>,
}

impl Drop for SearchThreadHandle {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl fmt::Debug for SearchThreadHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchThreadHandle")
            .field("thread_running", &self.join.is_some())
            .finish_non_exhaustive()
    }
}

fn push_error(sender: &AsyncStreamSender<PdfDocumentFindEvent>, error: PdfKitError) {
    sender.push(PdfDocumentFindEvent::Failed(error));
}

/// Async stream of `PDFDocument` find notifications and match snapshots.
#[derive(Debug)]
pub struct PdfDocumentFindStream {
    inner: BoundedAsyncStream<PdfDocumentFindEvent>,
    _handle: SearchThreadHandle,
}

impl PdfDocumentFindStream {
    /// Start an async document search.
    pub fn find_string(
        document: &PdfDocument,
        needle: &str,
        options: PdfDocumentFindOptions,
        capacity: usize,
    ) -> Result<Self> {
        if capacity == 0 {
            return Err(PdfKitError::new(
                ffi::status::INVALID_ARGUMENT,
                "async stream capacity must be > 0",
            ));
        }

        let needle = util::c_string(needle)?;
        let document_ptr = unsafe { ffi::pdf_object_retain(document.as_handle_ptr()) };
        if document_ptr.is_null() {
            return Err(PdfKitError::new(
                ffi::status::NULL_RESULT,
                "PDFDocument retain returned null",
            ));
        }

        let document_addr = document_ptr as usize;
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let join = thread::spawn(move || {
            let Some(handle) =
                (unsafe { ObjectHandle::from_retained_ptr(document_addr as *mut c_void) })
            else {
                push_error(
                    &sender,
                    PdfKitError::new(ffi::status::NULL_RESULT, "PDFDocument retain returned null"),
                );
                return;
            };
            let document = PdfDocument::from_handle(handle);

            sender.push(PdfDocumentFindEvent::Notification(
                PdfDocumentNotification::DidBeginFind,
            ));

            let mut out_error = ptr::null_mut();
            let json_ptr = unsafe {
                ffi::pdf_document_find_string_json(
                    document.as_handle_ptr(),
                    needle.as_ptr(),
                    options.bits(),
                    &mut out_error,
                )
            };

            let Some(json) = util::take_string(json_ptr) else {
                let message = util::take_string(out_error)
                    .unwrap_or_else(|| "PDFDocument.findString returned null".to_string());
                push_error(&sender, PdfKitError::new(ffi::status::FRAMEWORK, message));
                return;
            };

            match serde_json::from_str::<Vec<PdfDocumentFindMatch>>(&json) {
                Ok(matches) => {
                    for found in matches {
                        sender.push(PdfDocumentFindEvent::Match(found));
                    }
                    sender.push(PdfDocumentFindEvent::Notification(
                        PdfDocumentNotification::DidEndFind,
                    ));
                }
                Err(error) => push_error(
                    &sender,
                    PdfKitError::new(
                        ffi::status::FRAMEWORK,
                        format!("failed to parse PDFDocument find results: {error}"),
                    ),
                ),
            }
        });

        Ok(Self {
            inner: stream,
            _handle: SearchThreadHandle { join: Some(join) },
        })
    }

    /// Await the next find event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, PdfDocumentFindEvent> {
        self.inner.next()
    }

    /// Return the next buffered event without waiting.
    #[must_use]
    pub fn try_next(&self) -> Option<PdfDocumentFindEvent> {
        self.inner.try_next()
    }

    /// Return the number of buffered events.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Return `true` once the worker thread has finished and the stream has closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
