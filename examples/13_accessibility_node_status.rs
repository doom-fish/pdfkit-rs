use pdfkit::prelude::*;

fn main() {
    println!(
        "public_api_available={}",
        PdfAccessibilityNode::public_api_available()
    );
    println!("note={:?}", PdfAccessibilityNode::availability_note());
    println!("✅ pdfkit accessibility node OK");
}
