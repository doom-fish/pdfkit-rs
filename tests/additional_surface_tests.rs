mod common;

use pdfkit::prelude::*;

const TINY_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0,
    0, 1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 156,
    99, 248, 255, 255, 63, 0, 5, 254, 2, 254, 167, 53, 129, 132, 0, 0, 0, 0, 73, 69,
    78, 68, 174, 66, 96, 130,
];

struct NoopViewDelegate;

impl PdfViewDelegate for NoopViewDelegate {}

struct NoopPageOverlayProvider;

impl PdfPageOverlayViewProvider for NoopPageOverlayProvider {}

#[test]
fn new_string_and_utility_enums_are_exposed() {
    assert_eq!(PdfAnnotationSubtype::Link.name(), "/Link");
    assert_eq!(
        PdfAnnotationSubtype::from_name("/Widget"),
        Some(PdfAnnotationSubtype::Widget)
    );
    assert_eq!(PdfAnnotationHighlightingMode::Push.name(), "/P");
    assert_eq!(
        PdfAnnotationLineEndingStyle::from_line_style(PdfLineStyle::OpenArrow),
        PdfAnnotationLineEndingStyle::OpenArrow
    );
    assert_eq!(
        PdfAnnotationTextIconName::from_icon_type(PdfTextAnnotationIconType::Help),
        PdfAnnotationTextIconName::Help
    );
    assert_eq!(PdfAnnotationWidgetSubtype::Button.name(), "/Btn");
    assert_eq!(PdfAnnotationKey::WidgetValue.name(), "/V");

    assert_eq!(PdfTextAnnotationIconType::from_raw(6), Some(PdfTextAnnotationIconType::Insert));
    assert_eq!(PdfWidgetCellState::from_raw(-1), Some(PdfWidgetCellState::Mixed));
    assert_eq!(PdfPrintScalingMode::from_raw(2), Some(PdfPrintScalingMode::DownToFit));
    assert_eq!(
        PdfSelectionGranularity::from_raw(1),
        Some(PdfSelectionGranularity::Word)
    );
    assert_eq!(
        PdfThumbnailLayoutMode::from_raw(1),
        Some(PdfThumbnailLayoutMode::Horizontal)
    );

    let interest = PdfAreaOfInterest::TEXT | PdfAreaOfInterest::LINK;
    assert!(interest.contains(PdfAreaOfInterest::TEXT));
    assert!(interest.intersects(PdfAreaOfInterest::LINK));
    assert!(!interest.contains(PdfAreaOfInterest::IMAGE));

    let unspecified_value = std::hint::black_box(PdfDestination::UNSPECIFIED_VALUE);
    assert!(unspecified_value > 1.0e30);

    let options = PdfPageImageInitializationOptions::default()
        .with_media_box(PdfRect {
            x: 0.0,
            y: 0.0,
            width: 144.0,
            height: 72.0,
        })
        .with_rotation(90)
        .with_upscale_if_smaller(true)
        .with_compression_quality(0.75);
    assert!((options.media_box.unwrap().width - 144.0).abs() < f64::EPSILON);
    assert_eq!(options.rotation, Some(90));
    assert!(options.upscale_if_smaller);
    assert_eq!(options.compression_quality, Some(0.75));
}

#[test]
fn action_reset_form_roundtrips_fields() -> Result<()> {
    let action = PdfActionResetForm::new()?;
    assert_eq!(action.fields()?, Vec::<String>::new());
    assert!(action.fields_included_are_cleared());

    action.set_fields(["firstName", "lastName"])?;
    assert_eq!(action.fields()?, vec!["firstName".to_string(), "lastName".to_string()]);
    action.set_fields_included_are_cleared(false);
    assert!(!action.fields_included_are_cleared());

    let annotation = PdfAnnotation::new_with_subtype(
        PdfRect {
            x: 36.0,
            y: 36.0,
            width: 120.0,
            height: 40.0,
        },
        PdfAnnotationSubtype::Link,
    )?;
    annotation.set_action(Some(&action))?;
    let generic_action = annotation.action().expect("generic action");
    let reset_form = generic_action.as_reset_form().expect("reset-form action");
    assert_eq!(generic_action.action_type().as_deref(), Some("ResetForm"));
    assert_eq!(reset_form.fields()?, vec!["firstName".to_string(), "lastName".to_string()]);

    reset_form.clear_fields()?;
    assert!(reset_form.fields()?.is_empty());
    Ok(())
}

#[test]
fn page_image_initialization_and_view_delegate_handles_work() -> Result<()> {
    let media_box = PdfRect {
        x: 0.0,
        y: 0.0,
        width: 220.0,
        height: 110.0,
    };
    let page = PdfPage::from_image_data(
        TINY_PNG,
        &PdfPageImageInitializationOptions::default()
            .with_media_box(media_box)
            .with_upscale_if_smaller(true),
    )?;
    assert_eq!(page.bounds(DisplayBox::MediaBox), media_box);

    let view = PdfView::new(PdfSize {
        width: 320.0,
        height: 240.0,
    })?;
    let delegate = PdfViewDelegateHandle::new(NoopViewDelegate)?;
    view.set_delegate(Some(&delegate))?;

    let provider = PdfPageOverlayViewProviderHandle::new(NoopPageOverlayProvider)?;
    view.set_page_overlay_view_provider(Some(&provider))?;

    let _overlay = PdfPageOverlayView::new(PdfSize {
        width: 32.0,
        height: 24.0,
    })?;
    let _ = view.area_of_interest_for_point(common::word_point());
    Ok(())
}
