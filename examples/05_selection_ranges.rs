#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let page = document.page(0).ok_or_else(|| std::io::Error::other("missing page"))?;
    let hello = page
        .selection_for_range(0, 5)
        .ok_or_else(|| std::io::Error::other("missing range selection"))?;
    let selection = PdfSelection::new(&document)?;
    selection.add_selection(&hello)?;
    selection.extend_selection_for_line_boundaries();

    println!("selection={:?}", selection.string());
    println!("range={:?}", selection.text_range(0, &page));
    println!("by_line={}", selection.selection_by_line_count());
    println!("✅ pdfkit selection OK");
    Ok(())
}
