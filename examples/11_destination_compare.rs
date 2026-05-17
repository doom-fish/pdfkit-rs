#[path = "common/mod.rs"]
mod support;

use std::cmp::Ordering;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (_document, page) = support::empty_document_with_page()?;
    let destination = support::sample_destination(&page)?;
    destination.set_zoom(2.0);
    let comparison = destination.compare(&destination);
    if comparison != Ordering::Equal {
        return Err(std::io::Error::other("destination self-compare should be equal").into());
    }

    println!("destination={:?}", destination.info()?);
    println!("✅ pdfkit destination OK");
    Ok(())
}
