mod common;

use pdfkit::prelude::*;

#[test]
fn outline_tree_roundtrip() -> Result<()> {
    let (document, page) = common::empty_document_with_page()?;
    let root = PdfOutline::new()?;
    let child = PdfOutline::new()?;
    child.set_label(Some("Section 1"))?;
    let destination = common::sample_destination(&page)?;
    child.set_destination(Some(&destination))?;
    child.set_open(true);
    root.insert_child(&child, 0)?;
    document.set_outline_root(Some(&root))?;

    let fetched_root = document.outline_root().expect("root outline");
    let fetched_child = fetched_root.child(0).expect("child outline");
    assert_eq!(fetched_root.child_count(), 1);
    assert_eq!(fetched_child.label().as_deref(), Some("Section 1"));
    assert!(fetched_child.is_open());
    assert!(fetched_child.destination().is_some());
    Ok(())
}
