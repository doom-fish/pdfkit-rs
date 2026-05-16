mod common;

use std::path::Path;

use pdfkit::prelude::*;

fn file_url_string(path: &Path) -> String {
    format!("file://{}", path.display())
}

#[test]
fn action_named_roundtrip_via_generic_action_property() -> Result<()> {
    let action = PdfActionNamed::new(PdfActionNamedName::NextPage)?;
    assert_eq!(action.name(), Some(PdfActionNamedName::NextPage));
    action.set_name(PdfActionNamedName::LastPage)?;

    let annotation = PdfAnnotation::new(
        PdfRect {
            x: 24.0,
            y: 24.0,
            width: 120.0,
            height: 32.0,
        },
        "Link",
    )?;
    annotation.set_action(Some(&action))?;

    let generic = annotation.action().expect("action");
    assert!(generic.action_type().is_some());
    let named = generic.as_named().expect("named action");
    assert_eq!(named.name(), Some(PdfActionNamedName::LastPage));
    Ok(())
}

#[test]
fn action_remote_goto_roundtrip_via_generic_action_property() -> Result<()> {
    let remote_url = file_url_string(&common::fixture_path());
    let action = PdfActionRemoteGoTo::new(2, PdfPoint { x: 18.0, y: 24.0 }, &remote_url)?;
    assert_eq!(action.page_index(), 2);
    assert_eq!(action.point(), PdfPoint { x: 18.0, y: 24.0 });

    action.set_page_index(3);
    action.set_point(PdfPoint { x: 36.0, y: 72.0 });
    action.set_url(&remote_url)?;

    let outline = PdfOutline::new()?;
    outline.set_action(Some(&action))?;

    let generic = outline.action().expect("action");
    assert!(generic.action_type().is_some());
    let remote = generic.as_remote_goto().expect("remote action");
    assert_eq!(remote.page_index(), 3);
    assert_eq!(remote.point(), PdfPoint { x: 36.0, y: 72.0 });
    assert_eq!(remote.url().as_deref(), Some(remote_url.as_str()));
    Ok(())
}
