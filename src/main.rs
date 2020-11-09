use lsp_text_document::{FullTextDocument, compute_line_offsets, range_at};
use lsp_types::Url;

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
fn main() {
    let mut document = new_document("function abc() {\n  foo();\n  bar();\n  \n}");
    println!("{:?}", document.get_line_offsets());
    // assert_eq!(document.line_count(), 3);
    // assert_valid_line_number(&mut document);
}
