//! Executor-agnostic async wrappers for PDFKit document finding.
//!
//! Enabled with the `async` Cargo feature.
//!
//! [`PdfDocumentFindStream`] copies the document when the search starts and runs
//! `PDFDocument.findString(_:withOptions:)` on that copy on a background dispatch
//! queue, so the caller's document is never used from another thread. Every match
//! is converted into an owned Rust snapshot and emitted through a bounded async
//! stream backed by [`doom_fish_utils::stream::BoundedAsyncStream`].
//!
//! The stream emits synthetic `DidBeginFind` / `DidEndFind` notifications around
//! the match sequence. Every match is delivered: when `capacity` events are
//! buffered, delivery waits for the consumer. Dropping the stream does not wait
//! for the search; results that arrive afterwards are discarded.
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

use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::ops::BitOr;
use std::ptr;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};
use serde::Deserialize;

use crate::error::{PdfKitError, Result};
use crate::ffi;
use crate::util;
use crate::{PdfDocument, PdfDocumentNotification, PdfTextRange};

type FindSink = CallbackContext<Mutex<Option<AsyncStreamSender<PdfDocumentFindEvent>>>>;

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

fn push_error(sender: &AsyncStreamSender<PdfDocumentFindEvent>, error: PdfKitError) {
    let _ = sender.push_or_block(PdfDocumentFindEvent::Failed(error));
}

/// Async stream of `PDFDocument` find notifications and match snapshots.
#[derive(Debug)]
pub struct PdfDocumentFindStream {
    _sink: FindSink,
    inner: BoundedAsyncStream<PdfDocumentFindEvent>,
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
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        sender.push(PdfDocumentFindEvent::Notification(
            PdfDocumentNotification::DidBeginFind,
        ));
        let sink = FindSink::new(Mutex::new(Some(sender)));
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::pdf_document_find_string_async(
                document.as_handle_ptr(),
                needle.as_ptr(),
                options.bits(),
                find_result_trampoline,
                sink.as_ptr(),
                FindSink::RETAIN,
                FindSink::RELEASE,
                &raw mut out_error,
            )
        };
        util::status_result(status, out_error)?;

        Ok(Self {
            _sink: sink,
            inner: stream,
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

    /// Return `true` once the search has finished and the stream has closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

unsafe extern "C" fn find_result_trampoline(
    json: *const c_char,
    error: *const c_char,
    context: *mut c_void,
) {
    let _ = FindSink::with(context, "pdf_document_find_result", |sink| {
        let Some(sender) = sink.lock().unwrap_or_else(PoisonError::into_inner).take() else {
            return;
        };
        if !error.is_null() {
            let message = unsafe { CStr::from_ptr(error) }
                .to_string_lossy()
                .into_owned();
            push_error(&sender, PdfKitError::new(ffi::status::FRAMEWORK, message));
            return;
        }
        if json.is_null() {
            push_error(
                &sender,
                PdfKitError::new(
                    ffi::status::NULL_RESULT,
                    "PDFDocument.findString returned null",
                ),
            );
            return;
        }
        let json = unsafe { CStr::from_ptr(json) }.to_string_lossy();
        match serde_json::from_str::<Vec<PdfDocumentFindMatch>>(&json) {
            Ok(matches) => {
                let events = matches
                    .into_iter()
                    .map(PdfDocumentFindEvent::Match)
                    .chain([PdfDocumentFindEvent::Notification(
                        PdfDocumentNotification::DidEndFind,
                    )]);
                for event in events {
                    if sender.push_or_block(event).is_err() {
                        return;
                    }
                }
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
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::sync::Mutex;

    use doom_fish_utils::stream::BoundedAsyncStream;

    use super::{find_result_trampoline, FindSink, PdfDocumentFindEvent};
    use crate::PdfDocumentNotification;

    fn new_sink() -> (BoundedAsyncStream<PdfDocumentFindEvent>, FindSink) {
        let (stream, sender) = BoundedAsyncStream::new(8);
        (stream, FindSink::new(Mutex::new(Some(sender))))
    }

    #[test]
    fn results_are_delivered_once_and_close_the_stream() {
        let (stream, sink) = new_sink();
        let json =
            CString::new(r#"[{"text":"Hello","pages":[{"page_index":0,"ranges":[]}]}]"#).unwrap();

        unsafe {
            find_result_trampoline(json.as_ptr(), std::ptr::null(), sink.as_ptr());
            find_result_trampoline(json.as_ptr(), std::ptr::null(), sink.as_ptr());
        }

        assert!(matches!(
            stream.try_next(),
            Some(PdfDocumentFindEvent::Match(_))
        ));
        assert_eq!(
            stream.try_next(),
            Some(PdfDocumentFindEvent::Notification(
                PdfDocumentNotification::DidEndFind
            ))
        );
        assert_eq!(stream.try_next(), None);
        assert!(stream.is_closed());
    }

    #[test]
    fn errors_and_malformed_results_become_failed_events() {
        let (stream, sink) = new_sink();
        let message = CString::new("search failed").unwrap();
        unsafe { find_result_trampoline(std::ptr::null(), message.as_ptr(), sink.as_ptr()) };
        assert!(
            matches!(stream.try_next(), Some(PdfDocumentFindEvent::Failed(error)) if error.to_string().contains("search failed"))
        );
        assert!(stream.is_closed());

        let (stream, sink) = new_sink();
        let malformed = CString::new("{").unwrap();
        unsafe { find_result_trampoline(malformed.as_ptr(), std::ptr::null(), sink.as_ptr()) };
        assert!(matches!(
            stream.try_next(),
            Some(PdfDocumentFindEvent::Failed(_))
        ));
    }

    #[test]
    fn results_after_the_stream_is_dropped_are_discarded() {
        let (stream, sink) = new_sink();
        let swift_reference = sink.retained_ptr();
        drop(stream);
        drop(sink);

        let json = CString::new("[]").unwrap();
        unsafe {
            find_result_trampoline(json.as_ptr(), std::ptr::null(), swift_reference);
            (FindSink::RELEASE)(swift_reference);
        }
    }
}
