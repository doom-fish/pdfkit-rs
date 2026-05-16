#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let page = document.page(0).ok_or_else(|| std::io::Error::other("missing page"))?;
    let selection = page
        .selection_for_range(0, 5)
        .ok_or_else(|| std::io::Error::other("missing selection"))?;
    let view = PdfView::new(PdfSize {
        width: 320.0,
        height: 480.0,
    })?;
    view.set_document(Some(&document))?;
    view.set_display_mode(PdfDisplayMode::SinglePage)?;
    view.set_display_direction(PdfDisplayDirection::Vertical)?;
    view.set_display_box(DisplayBox::CropBox)?;
    view.set_auto_scales(true);
    view.layout_document_view();
    view.go_to_page(&page)?;
    view.set_current_selection(Some(&selection), false)?;

    println!("view={:?}", view.info()?);
    println!("visible_pages={}", view.visible_page_count());
    println!("✅ pdfkit view OK");
    Ok(())
}
