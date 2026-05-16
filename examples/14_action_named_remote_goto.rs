#[path = "common/mod.rs"]
mod support;

use std::path::Path;

use pdfkit::prelude::*;

fn file_url_string(path: &Path) -> String {
    format!("file://{}", path.display())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let named = PdfActionNamed::new(PdfActionNamedName::NextPage)?;
    let remote = PdfActionRemoteGoTo::new(
        0,
        PdfPoint { x: 18.0, y: 24.0 },
        &file_url_string(&support::fixture_path()),
    )?;

    let annotation = PdfAnnotation::new(
        PdfRect {
            x: 24.0,
            y: 24.0,
            width: 120.0,
            height: 32.0,
        },
        "Link",
    )?;
    annotation.set_action(Some(&remote))?;

    let outline = PdfOutline::new()?;
    outline.set_action(Some(&named))?;

    println!(
        "named={:?} remote_url={:?}",
        outline.action().and_then(|action| action.as_named()).and_then(|action| action.name()),
        annotation.action().and_then(|action| action.as_remote_goto()).and_then(|action| action.url())
    );
    println!("✅ pdfkit named/remote actions OK");
    Ok(())
}
