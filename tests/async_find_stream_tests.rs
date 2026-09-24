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

fn collect_events(stream: &PdfDocumentFindStream) -> Vec<PdfDocumentFindEvent> {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut events = Vec::new();
    while Instant::now() < deadline {
        while let Some(event) = stream.try_next() {
            events.push(event);
        }
        if stream.is_closed() {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }
    events
}

#[test]
fn async_find_searches_a_snapshot_while_the_document_is_mutated() -> Result<()> {
    let document = common::fixture_document()?;
    let stream =
        PdfDocumentFindStream::find_string(&document, "Hello", PdfDocumentFindOptions::NONE, 16)?;
    document.remove_page(0)?;
    assert_eq!(document.page_count(), 0);

    let events = collect_events(&stream);
    assert!(stream.is_closed(), "stream should close after the search");
    let matches: Vec<_> = events
        .iter()
        .filter_map(|event| match event {
            PdfDocumentFindEvent::Match(found) => Some(found),
            _ => None,
        })
        .collect();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].pages[0].page_index, 0);
    assert_eq!(
        events.last(),
        Some(&PdfDocumentFindEvent::Notification(
            PdfDocumentNotification::DidEndFind
        ))
    );
    Ok(())
}

#[test]
fn dropping_a_find_stream_before_results_arrive_is_harmless() -> Result<()> {
    let document = common::fixture_document()?;
    for _ in 0..16 {
        drop(PdfDocumentFindStream::find_string(
            &document,
            "Hello",
            PdfDocumentFindOptions::NONE,
            1,
        )?);
    }
    let stream =
        PdfDocumentFindStream::find_string(&document, "Hello", PdfDocumentFindOptions::NONE, 16)?;
    assert!(collect_events(&stream)
        .iter()
        .any(|event| matches!(event, PdfDocumentFindEvent::Match(_))));
    Ok(())
}

#[test]
fn find_streams_can_move_between_threads() {
    fn assert_send<T: Send>() {}
    assert_send::<PdfDocumentFindStream>();
}

#[test]
fn a_full_buffer_delays_matches_instead_of_dropping_them() -> Result<()> {
    let document = common::fixture_document()?;
    let expected = document
        .page(0)
        .and_then(|page| page.string())
        .unwrap_or_default()
        .matches('l')
        .count();
    assert!(expected > 1);

    let stream =
        PdfDocumentFindStream::find_string(&document, "l", PdfDocumentFindOptions::NONE, 1)?;
    thread::sleep(Duration::from_millis(200));
    let events = collect_events(&stream);

    let matches = events
        .iter()
        .filter(|event| matches!(event, PdfDocumentFindEvent::Match(_)))
        .count();
    assert_eq!(matches, expected);
    assert_eq!(
        events.first(),
        Some(&PdfDocumentFindEvent::Notification(
            PdfDocumentNotification::DidBeginFind
        ))
    );
    assert_eq!(
        events.last(),
        Some(&PdfDocumentFindEvent::Notification(
            PdfDocumentNotification::DidEndFind
        ))
    );
    Ok(())
}
