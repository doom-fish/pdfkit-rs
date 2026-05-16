mod common;

use pdfkit::prelude::*;

#[test]
fn thumbnail_view_roundtrip() -> Result<()> {
    let document = common::fixture_document()?;
    let view = PdfView::new(PdfSize { width: 320.0, height: 480.0 })?;
    view.set_document(Some(&document))?;
    view.layout_document_view();

    let thumbnails = PdfThumbnailView::new(PdfSize { width: 180.0, height: 360.0 })?;
    thumbnails.set_pdf_view(Some(&view))?;
    thumbnails.set_thumbnail_size(PdfSize { width: 96.0, height: 128.0 });
    thumbnails.set_maximum_number_of_columns(2);
    thumbnails.set_allows_dragging(true);
    thumbnails.set_allows_multiple_selection(true);

    let info = thumbnails.info()?;
    assert!(info.has_pdf_view);
    assert!((info.thumbnail_size.width - 96.0).abs() < f64::EPSILON);
    assert_eq!(info.maximum_number_of_columns, 2);
    Ok(())
}
