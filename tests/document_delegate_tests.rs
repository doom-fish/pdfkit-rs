mod common;

use std::cell::RefCell;
use std::rc::Rc;

use pdfkit::prelude::*;

#[derive(Default)]
struct DelegateCounts {
    page_class_requests: usize,
    annotation_class_requests: usize,
}

struct CountingDelegate {
    counts: Rc<RefCell<DelegateCounts>>,
}

impl PdfDocumentDelegate for CountingDelegate {
    fn page_class_name(&mut self) -> Option<String> {
        self.counts.borrow_mut().page_class_requests += 1;
        Some("PDFPage".to_string())
    }

    fn annotation_class_name(&mut self, annotation_type: &str) -> Option<String> {
        if annotation_type == "Link" {
            self.counts.borrow_mut().annotation_class_requests += 1;
            Some("PDFAnnotation".to_string())
        } else {
            None
        }
    }
}

#[test]
fn document_delegate_can_override_page_and_annotation_classes() -> Result<()> {
    let source = PdfDocument::new()?;
    let page = PdfPage::new()?;
    source.insert_page(&page, 0)?;
    let annotation = common::sample_link_annotation()?;
    page.add_annotation(&annotation)?;
    let output = common::output_path("document-delegate.pdf");
    source.write_to_url(&output)?;

    let counts = Rc::new(RefCell::new(DelegateCounts::default()));
    let delegate = PdfDocumentDelegateHandle::new(CountingDelegate {
        counts: Rc::clone(&counts),
    })?;

    let document = PdfDocument::from_url(&output)?;
    document.set_delegate(Some(&delegate))?;
    let reopened_page = document.page(0).expect("page");
    assert_eq!(reopened_page.annotation_count(), 1);
    let reopened_annotation = reopened_page.annotation(0).expect("annotation");
    assert_eq!(
        reopened_annotation.info()?.annotation_type.as_deref(),
        Some("Link")
    );

    let counts = counts.borrow();
    assert!(counts.page_class_requests > 0);
    assert!(counts.annotation_class_requests > 0);
    Ok(())
}
