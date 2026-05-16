mod common;

use pdfkit::prelude::*;

#[test]
fn document_open_and_write_roundtrip() -> Result<()> {
    let document = common::fixture_document()?;
    assert_eq!(document.page_count(), 1);
    assert!(document.string().unwrap_or_default().contains("Hello PDFKit"));

    let output = common::output_path("document-roundtrip.pdf");
    document.write_to_url(&output)?;
    let reopened = PdfDocument::from_url(&output)?;
    assert_eq!(reopened.page_count(), 1);
    Ok(())
}

#[test]
fn empty_document_page_management() -> Result<()> {
    let (document, _page) = common::empty_document_with_page()?;
    let extra = PdfPage::new()?;
    document.insert_page(&extra, 1)?;
    assert_eq!(document.page_count(), 2);
    assert_eq!(document.page_index(&extra), Some(1));
    assert!(document.exchange_pages(0, 1).is_err());
    assert_eq!(document.page_count(), 2);
    document.remove_page(1)?;
    assert_eq!(document.page_count(), 1);
    Ok(())
}
