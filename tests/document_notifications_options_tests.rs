mod common;

use pdfkit::prelude::*;

#[test]
fn notifications_new_enums_and_write_options_builders_are_exposed() {
    assert_eq!(
        [
            PdfDocumentNotification::DidUnlock.name(),
            PdfDocumentNotification::DidBeginFind.name(),
            PdfDocumentNotification::DidEndFind.name(),
            PdfDocumentNotification::DidBeginPageFind.name(),
            PdfDocumentNotification::DidEndPageFind.name(),
            PdfDocumentNotification::DidFindMatch.name(),
            PdfDocumentNotification::DidBeginWrite.name(),
            PdfDocumentNotification::DidEndWrite.name(),
            PdfDocumentNotification::DidBeginPageWrite.name(),
            PdfDocumentNotification::DidEndPageWrite.name(),
        ],
        [
            "PDFDocumentDidUnlockNotification",
            "PDFDocumentDidBeginFindNotification",
            "PDFDocumentDidEndFindNotification",
            "PDFDocumentDidBeginPageFindNotification",
            "PDFDocumentDidEndPageFindNotification",
            "PDFDocumentDidFindMatchNotification",
            "PDFDocumentDidBeginWriteNotification",
            "PDFDocumentDidEndWriteNotification",
            "PDFDocumentDidBeginPageWriteNotification",
            "PDFDocumentDidEndPageWriteNotification",
        ]
    );
    assert_eq!(
        [
            PdfDocumentNotificationUserInfoKey::FoundSelection.name(),
            PdfDocumentNotificationUserInfoKey::PageIndex.name(),
        ],
        ["PDFDocumentFoundSelection", "PDFDocumentPageIndex"]
    );
    assert_eq!(
        [
            PdfViewNotification::AnnotationHit.name(),
            PdfViewNotification::AnnotationWillHit.name(),
            PdfViewNotification::ChangedHistory.name(),
            PdfViewNotification::CopyPermission.name(),
            PdfViewNotification::DisplayBoxChanged.name(),
            PdfViewNotification::DisplayModeChanged.name(),
            PdfViewNotification::DocumentChanged.name(),
            PdfViewNotification::PageChanged.name(),
            PdfViewNotification::PrintPermission.name(),
            PdfViewNotification::ScaleChanged.name(),
            PdfViewNotification::SelectionChanged.name(),
            PdfViewNotification::VisiblePagesChanged.name(),
        ],
        [
            "PDFViewAnnotationHitNotification",
            "PDFViewAnnotationWillHitNotification",
            "PDFViewChangedHistoryNotification",
            "PDFViewCopyPermissionNotification",
            "PDFViewDisplayBoxChangedNotification",
            "PDFViewDisplayModeChangedNotification",
            "PDFViewDocumentChangedNotification",
            "PDFViewPageChangedNotification",
            "PDFViewPrintPermissionNotification",
            "PDFViewScaleChangedNotification",
            "PDFViewSelectionChangedNotification",
            "PDFViewVisiblePagesChangedNotification",
        ]
    );
    assert_eq!(
        PdfThumbnailViewNotification::DocumentEdited.name(),
        "PDFThumbnailViewDocumentEditedNotification"
    );
    assert_eq!(PdfActionNamedName::from_raw(8), Some(PdfActionNamedName::Find));
    assert_eq!(PdfActionNamedName::ZoomOut.as_raw(), 11);
    assert_eq!(PdfLineStyle::from_raw(4), Some(PdfLineStyle::OpenArrow));
    assert_eq!(PdfMarkupType::from_raw(3), Some(PdfMarkupType::Redact));

    let options = PdfDocumentWriteOptions::default()
        .with_owner_password("owner")
        .with_user_password("user")
        .with_access_permissions(1)
        .with_burn_in_annotations(true)
        .with_save_text_from_ocr(true)
        .with_save_images_as_jpeg(true)
        .with_optimize_images_for_screen(true);
    assert_eq!(options.owner_password.as_deref(), Some("owner"));
    assert_eq!(options.user_password.as_deref(), Some("user"));
    assert_eq!(options.access_permissions, Some(1));
    assert!(options.burn_in_annotations);
    assert!(options.save_text_from_ocr);
    assert!(options.save_images_as_jpeg);
    assert!(options.optimize_images_for_screen);
}

#[test]
fn document_write_options_roundtrip() -> Result<()> {
    let (document, page) = common::empty_document_with_page()?;
    let annotation = common::sample_link_annotation()?;
    page.add_annotation(&annotation)?;
    let output = common::output_path("document-write-options.pdf");
    let options = PdfDocumentWriteOptions::default()
        .with_owner_password("owner")
        .with_user_password("user")
        .with_access_permissions(1);

    document.write_to_url_with_options(&output, &options)?;

    let reopened = PdfDocument::from_url(&output)?;
    let info = reopened.info()?;
    assert!(info.is_encrypted);
    assert!(reopened.unlock("user")? || reopened.unlock("owner")?);
    let unlocked_info = reopened.info()?;
    assert_ne!(unlocked_info.access_permissions & 1, 0);
    Ok(())
}
