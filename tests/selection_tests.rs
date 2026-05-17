mod common;

use pdfkit::prelude::*;

#[test]
fn selection_ranges_and_line_splits() -> Result<()> {
    let document = common::fixture_document()?;
    let page = document.page(0).expect("first page");
    let hello = page.selection_for_range(0, 5).expect("hello selection");
    let selection = PdfSelection::new(&document)?;
    selection.add_selection(&hello)?;
    assert_eq!(selection.string().as_deref(), Some("Hello"));
    assert_eq!(
        selection.text_range(0, &page),
        Some(PdfTextRange {
            location: 0,
            length: 5
        })
    );
    assert!(selection.selection_by_line_count() >= 1);
    Ok(())
}
