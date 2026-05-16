#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let view = PdfView::new(PdfSize {
        width: 320.0,
        height: 480.0,
    })?;
    view.set_document(Some(&document))?;
    view.layout_document_view();

    let thumbnails = PdfThumbnailView::new(PdfSize {
        width: 180.0,
        height: 360.0,
    })?;
    thumbnails.set_pdf_view(Some(&view))?;
    thumbnails.set_thumbnail_size(PdfSize {
        width: 96.0,
        height: 128.0,
    });
    thumbnails.set_maximum_number_of_columns(2);
    thumbnails.set_allows_dragging(true);
    thumbnails.set_allows_multiple_selection(true);

    println!("thumbnail_view={:?}", thumbnails.info()?);
    println!("selected_pages={}", thumbnails.selected_page_count());
    println!("✅ pdfkit thumbnail view OK");
    Ok(())
}
