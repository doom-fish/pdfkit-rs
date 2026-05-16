mod common;

use pdfkit::prelude::*;

#[test]
fn action_url_roundtrip() -> Result<()> {
    let action = common::sample_url_action()?;
    assert_eq!(action.url().as_deref(), Some("https://example.com"));
    action.set_url("https://example.org")?;
    assert_eq!(action.url().as_deref(), Some("https://example.org"));
    assert!(action.action_type().is_some());
    Ok(())
}
