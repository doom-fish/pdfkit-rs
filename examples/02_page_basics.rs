#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let page = document.page(0).ok_or_else(|| std::io::Error::other("missing first page"))?;
    let bounds = page.bounds(DisplayBox::CropBox);
    let selection = page
        .selection_for_word_at_point(support::word_point())
        .ok_or_else(|| std::io::Error::other("missing word selection"))?;
    let first_char = page.character_bounds_at(0);

    println!("label={:?} chars={} bounds={bounds:?}", page.label(), page.number_of_characters());
    println!("word={:?} first_char={first_char:?}", selection.string());
    println!("✅ pdfkit page OK");
    Ok(())
}
