#[path = "common/mod.rs"]
mod support;

use pdfkit::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let appearance = PdfAppearanceCharacteristics::new()?;
    appearance.set_control_type(PdfWidgetControlType::PushButton)?;
    appearance.set_rotation(90);
    appearance.set_caption(Some("Open"))?;
    appearance.set_rollover_caption(Some("Hover"))?;
    appearance.set_down_caption(Some("Pressed"))?;
    appearance.set_background_color(PdfColor {
        red: 0.2,
        green: 0.3,
        blue: 0.4,
        alpha: 1.0,
    });
    appearance.set_border_color(PdfColor {
        red: 0.7,
        green: 0.2,
        blue: 0.1,
        alpha: 1.0,
    });

    println!("appearance={:?}", appearance.info()?);
    println!("✅ pdfkit appearance OK");
    Ok(())
}
