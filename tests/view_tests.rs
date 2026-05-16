mod common;

use pdfkit::prelude::*;

#[test]
fn pdf_view_tracks_document_and_selection() -> Result<()> {
    let document = common::fixture_document()?;
    let page = document.page(0).expect("page");
    let selection = page.selection_for_range(0, 5).expect("selection");
    let view = PdfView::new(PdfSize { width: 320.0, height: 480.0 })?;
    view.set_document(Some(&document))?;
    view.set_display_mode(PdfDisplayMode::SinglePage)?;
    view.set_display_direction(PdfDisplayDirection::Vertical)?;
    view.set_display_box(DisplayBox::CropBox)?;
    view.set_auto_scales(true);
    view.layout_document_view();
    view.go_to_page(&page)?;
    view.set_current_selection(Some(&selection), false)?;

    let info = view.info()?;
    assert!(view.document().is_some());
    assert!(view.current_page().is_some());
    assert!(view.current_selection().is_some());
    assert_eq!(view.visible_page_count(), info.visible_page_count);
    Ok(())
}
