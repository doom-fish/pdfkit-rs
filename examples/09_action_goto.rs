#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (_document, page) = support::empty_document_with_page()?;
    let destination = support::sample_destination(&page)?;
    let action = PdfActionGoTo::new(&destination)?;

    println!("type={:?} destination={:?}", action.action_type(), action.destination().and_then(|value| value.info().ok()));
    println!("✅ pdfkit action goto OK");
    Ok(())
}
