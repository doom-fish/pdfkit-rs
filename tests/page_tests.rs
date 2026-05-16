mod common;

use pdfkit::prelude::*;

#[test]
fn page_bounds_and_selection_helpers() -> Result<()> {
    let page = common::fixture_page()?;
    assert!(page.bounds(DisplayBox::CropBox).width > 0.0);
    assert_eq!(page.selection_for_word_at_point(common::word_point()).and_then(|selection| selection.string()), Some("Hello".to_string()));
    assert!(page.selection_for_line_at_point(common::word_point()).and_then(|selection| selection.string()).unwrap_or_default().contains("Hello PDFKit"));
    let first_character = page.character_bounds_at(0);
    let midpoint = PdfPoint {
        x: first_character.x + (first_character.width / 2.0),
        y: first_character.y + (first_character.height / 2.0),
    };
    assert_eq!(page.character_index_at_point(midpoint), Some(0));
    Ok(())
}

#[test]
fn empty_page_mutation_roundtrip() -> Result<()> {
    let page = PdfPage::new()?;
    page.set_bounds(
        DisplayBox::MediaBox,
        PdfRect {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 300.0,
        },
    )?;
    page.set_rotation(90)?;
    page.set_displays_annotations(false);
    assert_eq!(page.rotation(), 90);
    assert!(!page.displays_annotations());
    assert!((page.bounds(DisplayBox::MediaBox).width - 200.0).abs() < f64::EPSILON);
    Ok(())
}
