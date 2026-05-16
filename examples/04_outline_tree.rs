#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (document, page) = support::empty_document_with_page()?;
    let root = PdfOutline::new()?;
    let child = PdfOutline::new()?;
    child.set_label(Some("Section 1"))?;
    let destination = support::sample_destination(&page)?;
    child.set_destination(Some(&destination))?;
    root.insert_child(&child, 0)?;
    document.set_outline_root(Some(&root))?;

    let fetched_root = document.outline_root().ok_or_else(|| std::io::Error::other("missing root outline"))?;
    let fetched_child = fetched_root.child(0).ok_or_else(|| std::io::Error::other("missing child outline"))?;

    println!("root_children={} child_label={:?}", fetched_root.child_count(), fetched_child.label());
    println!("destination={:?}", fetched_child.destination().and_then(|value| value.info().ok()));
    println!("✅ pdfkit outline OK");
    Ok(())
}
