#[path = "common/mod.rs"]
mod support;

use std::sync::{Arc, Mutex};

use pdfkit::prelude::*;

#[derive(Default)]
struct DelegateCounts {
    page_class_requests: usize,
}

struct ExampleDelegate {
    counts: Arc<Mutex<DelegateCounts>>,
}

impl PdfDocumentDelegate for ExampleDelegate {
    fn page_class_name(&mut self) -> Option<String> {
        self.counts.lock().unwrap().page_class_requests += 1;
        Some("PDFPage".to_string())
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let counts = Arc::new(Mutex::new(DelegateCounts::default()));
    let delegate = PdfDocumentDelegateHandle::new(ExampleDelegate {
        counts: Arc::clone(&counts),
    })?;

    let document = support::fixture_document()?;
    document.set_delegate(Some(&delegate))?;
    let _page = document.page(0);

    let output = support::output_dir().join("document-delegate-write-options.pdf");
    let options = PdfDocumentWriteOptions::default()
        .with_owner_password("owner")
        .with_user_password("user");
    document.write_to_url_with_options(&output, &options)?;

    println!(
        "page_class_requests={} did_unlock_notification={} view_page_changed_notification={}",
        counts.lock().unwrap().page_class_requests,
        PdfDocumentNotification::DidUnlock.name(),
        PdfViewNotification::PageChanged.name(),
    );
    println!("✅ pdfkit document delegate/write options OK");
    Ok(())
}
