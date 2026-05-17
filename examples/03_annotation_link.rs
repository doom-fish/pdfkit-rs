#[path = "common/mod.rs"]
mod support;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (_document, page) = support::empty_document_with_page()?;
    let annotation = support::sample_link_annotation()?;
    page.add_annotation(&annotation)?;
    let stored = page
        .annotation(0)
        .ok_or_else(|| std::io::Error::other("missing annotation"))?;
    let info = stored.info()?;
    let url = stored
        .action_url()
        .and_then(|action| action.url())
        .ok_or_else(|| std::io::Error::other("missing annotation action URL"))?;

    println!("annotation={info:?}");
    println!("url={url}");
    println!("✅ pdfkit annotation OK");
    Ok(())
}
