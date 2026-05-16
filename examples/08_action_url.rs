#[path = "common/mod.rs"]
mod support;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let action = support::sample_url_action()?;
    action.set_url("https://example.org/path")?;

    println!("type={:?} url={:?}", action.action_type(), action.url());
    println!("✅ pdfkit action url OK");
    Ok(())
}
