#[path = "common/mod.rs"]
mod support;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let border = support::sample_border()?;
    println!("border={:?}", border.info()?);
    println!("✅ pdfkit border OK");
    Ok(())
}
