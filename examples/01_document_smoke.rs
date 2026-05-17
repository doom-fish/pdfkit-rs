#[path = "common/mod.rs"]
mod support;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let document = support::fixture_document()?;
    let info = document.info()?;
    let output = support::output_dir().join("hello-copy.pdf");
    document.write_to_url(&output)?;

    if document.page_count() != 1 {
        return Err(std::io::Error::other("unexpected page count").into());
    }
    if !output.exists() {
        return Err(std::io::Error::other("expected written document").into());
    }

    println!(
        "pages={} encrypted={} page_class={}",
        document.page_count(),
        info.is_encrypted,
        info.page_class
    );
    println!("attributes={:?}", document.attributes()?);
    println!("✅ pdfkit document OK");
    Ok(())
}
