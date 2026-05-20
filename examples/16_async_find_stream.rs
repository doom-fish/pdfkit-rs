#[path = "common/mod.rs"]
mod support;

use std::future::poll_fn;
use std::time::{Duration, Instant};

use pdfkit::async_api::{PdfDocumentFindEvent, PdfDocumentFindOptions, PdfDocumentFindStream};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let stream =
        PdfDocumentFindStream::find_string(&document, "Hello", PdfDocumentFindOptions::NONE, 8)?;

    println!("Searching for \"Hello\" for up to 2 s…");
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut saw_match = false;

    pollster::block_on(async {
        loop {
            while let Some(event) = stream.try_next() {
                match event {
                    PdfDocumentFindEvent::Notification(notification) => {
                        println!("notification={}", notification.name());
                    }
                    PdfDocumentFindEvent::Match(found) => {
                        saw_match = true;
                        println!("match={:?} pages={:?}", found.text, found.pages);
                    }
                    PdfDocumentFindEvent::Failed(error) => {
                        eprintln!("search failed: {error}");
                        return;
                    }
                }
            }

            if stream.is_closed() || Instant::now() >= deadline {
                break;
            }

            poll_fn(|cx| {
                cx.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
    });

    if !saw_match {
        println!("No match arrived within the timeout.");
    }
    println!("Stream closed={}", stream.is_closed());
    Ok(())
}
