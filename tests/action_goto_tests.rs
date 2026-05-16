mod common;

use pdfkit::prelude::*;

#[test]
fn action_goto_roundtrip() -> Result<()> {
    let (_document, page) = common::empty_document_with_page()?;
    let destination = common::sample_destination(&page)?;
    let action = PdfActionGoTo::new(&destination)?;
    let fetched_destination = action.destination().expect("destination");
    assert_eq!(fetched_destination.info()?.page_index, Some(0));
    assert!(action.action_type().is_some());
    Ok(())
}
