mod common;

use std::thread;
use std::time::{Duration, Instant};

use pdfkit::async_api::{PdfDocumentFindEvent, PdfDocumentFindOptions, PdfDocumentFindStream};
use pdfkit::prelude::*;

#[test]
fn async_find_stream_reports_matches() -> Result<()> {
    let document = common::fixture_document()?;
    let stream =
        PdfDocumentFindStream::find_string(&document, "Hello", PdfDocumentFindOptions::NONE, 16)?;

    let deadline = Instant::now() + Duration::from_secs(2);
    let mut saw_begin = false;
    let mut saw_end = false;
    let mut matches = Vec::new();

    while Instant::now() < deadline {
        while let Some(event) = stream.try_next() {
            match event {
                PdfDocumentFindEvent::Notification(PdfDocumentNotification::DidBeginFind) => {
                    saw_begin = true;
                }
                PdfDocumentFindEvent::Notification(PdfDocumentNotification::DidEndFind) => {
                    saw_end = true;
                }
                PdfDocumentFindEvent::Notification(_) => {}
                PdfDocumentFindEvent::Match(found) => {
                    matches.push(found.text.unwrap_or_default());
                }
                PdfDocumentFindEvent::Failed(error) => return Err(error),
            }
        }

        if saw_end && stream.is_closed() {
            break;
        }

        thread::sleep(Duration::from_millis(10));
    }

    assert!(saw_begin, "expected DidBeginFind notification");
    assert!(saw_end, "expected DidEndFind notification");
    assert!(
        stream.is_closed(),
        "expected stream to close after DidEndFind"
    );
    assert!(matches.iter().any(|text| text.contains("Hello")));
    Ok(())
}
