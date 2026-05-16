mod common;

use pdfkit::prelude::*;

#[test]
fn annotation_can_attach_border_and_url_action() -> Result<()> {
    let (_document, page) = common::empty_document_with_page()?;
    let annotation = common::sample_link_annotation()?;
    page.add_annotation(&annotation)?;
    let stored = page.annotation(0).expect("stored annotation");
    let info = stored.info()?;
    assert_eq!(page.annotation_count(), 1);
    assert_eq!(info.contents.as_deref(), Some("Example link"));
    assert_eq!(stored.action_url().and_then(|action| action.url()), Some("https://example.com".to_string()));
    assert!(page.annotation_at_point(PdfPoint { x: 50.0, y: 50.0 }).is_some());
    Ok(())
}
