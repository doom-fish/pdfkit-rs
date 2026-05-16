mod common;

use pdfkit::prelude::*;

#[test]
fn border_style_roundtrip() -> Result<()> {
    let border = common::sample_border()?;
    let info = border.info()?;
    assert_eq!(info.style_enum(), Some(PdfBorderStyle::Dashed));
    assert!((info.line_width - 2.0).abs() < f64::EPSILON);
    let dash_pattern = info.dash_pattern.expect("dash pattern");
    assert_eq!(dash_pattern.len(), 2);
    assert!((dash_pattern[0] - 3.0).abs() < f64::EPSILON);
    assert!((dash_pattern[1] - 1.0).abs() < f64::EPSILON);
    Ok(())
}
