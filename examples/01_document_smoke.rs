use std::path::PathBuf;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/hello.pdf");
    let document = PdfDocument::from_url(&path)?;
    let text = document
        .string()
        .ok_or_else(|| std::io::Error::other("document string was empty"))?;
    let first_page = document
        .page(0)
        .ok_or_else(|| std::io::Error::other("missing first page"))?;
    let bounds = first_page.bounds(DisplayBox::CropBox);

    if document.page_count() != 1 {
        return Err(std::io::Error::other("unexpected page count").into());
    }
    if !text.contains("Hello PDFKit") {
        return Err(std::io::Error::other("expected fixture text missing").into());
    }
    if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return Err(std::io::Error::other("invalid page bounds").into());
    }

    println!("page count: {}", document.page_count());
    println!("page label: {:?}", first_page.label());
    println!("page bounds: {bounds:?}");
    println!("✅ pdfkit document OK");
    Ok(())
}
