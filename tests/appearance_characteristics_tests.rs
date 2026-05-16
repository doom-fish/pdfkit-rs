mod common;

use pdfkit::prelude::*;

#[test]
fn appearance_characteristics_roundtrip() -> Result<()> {
    let appearance = PdfAppearanceCharacteristics::new()?;
    appearance.set_control_type(PdfWidgetControlType::PushButton)?;
    appearance.set_rotation(90);
    appearance.set_caption(Some("Open"))?;
    appearance.set_rollover_caption(Some("Hover"))?;
    appearance.set_down_caption(Some("Pressed"))?;
    appearance.set_background_color(PdfColor { red: 0.2, green: 0.3, blue: 0.4, alpha: 1.0 });
    appearance.set_border_color(PdfColor { red: 0.7, green: 0.2, blue: 0.1, alpha: 1.0 });

    let info = appearance.info()?;
    assert_eq!(info.control_type_enum(), Some(PdfWidgetControlType::PushButton));
    assert_eq!(info.caption.as_deref(), Some("Open"));
    assert_eq!(info.rotation, 90);
    assert!(info.background_color.is_some());
    assert!(info.border_color.is_some());
    Ok(())
}
