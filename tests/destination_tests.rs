mod common;

use std::cmp::Ordering;

use pdfkit::prelude::*;

#[test]
fn destination_compare_and_page_access() -> Result<()> {
    let (_document, page) = common::empty_document_with_page()?;
    let destination = common::sample_destination(&page)?;
    destination.set_zoom(2.0);
    assert_eq!(
        destination
            .page()
            .and_then(|page| page.document())
            .map(|document| document.page_count()),
        Some(1)
    );
    assert_eq!(destination.compare(&destination), Ordering::Equal);
    assert_eq!(destination.info()?.page_index, Some(0));
    Ok(())
}
