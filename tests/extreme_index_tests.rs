mod common;

use pdfkit::prelude::*;

#[test]
fn out_of_range_character_indexes_return_none_instead_of_trapping() -> Result<()> {
    let document = common::fixture_document()?;
    let page = document.page(0).expect("page");
    let length = page.number_of_characters();

    assert!(page.selection_for_range(usize::MAX, 1).is_none());
    assert!(page.selection_for_range(0, usize::MAX).is_none());
    assert!(page.selection_for_range(isize::MAX as usize, 1).is_none());
    assert!(document
        .selection_from_page_characters(&page, usize::MAX, &page, 0)
        .is_none());
    assert!(document
        .selection_from_page_characters(&page, 0, &page, usize::MAX)
        .is_none());

    let selection = page.selection_for_range(0, length).expect("whole page");
    assert_eq!(selection.string().as_deref(), page.string().as_deref());
    Ok(())
}

#[test]
fn remote_goto_page_index_saturates_instead_of_trapping() -> Result<()> {
    let url = format!("file://{}", common::fixture_path().display());
    let action = PdfActionRemoteGoTo::new(usize::MAX, PdfPoint { x: 0.0, y: 0.0 }, &url)?;
    assert_eq!(action.page_index(), isize::MAX as usize);

    action.set_page_index(usize::MAX - 1);
    assert_eq!(action.page_index(), isize::MAX as usize);

    action.set_page_index(3);
    assert_eq!(action.page_index(), 3);
    Ok(())
}
