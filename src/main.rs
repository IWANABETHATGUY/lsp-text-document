use lsp_text_document::{FullTextDocument, compute_line_offsets, event, position, range_at, range};
use lsp_types::{ Url, Position };

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
fn main() {
    let mut document = new_document("我的你的真的加载\ntest");
    let mut string = "我的你的".to_string();
    println!("{:?}", string.chars().take(1).chain(string.chars().skip(3)).to_owned().collect::<String>());
    // document.update(vec![event!("abc123", range!(3, 0, 6, 10))], 2);
    println!("{:?}", document.get_line_offsets());
    // assert_eq!(document.text, "foo\nbarabc123");
    // assert_eq!(document.version, 2);
    // assert_eq!(document.offset_at(position!(1, 100)), 13);

    // assert_eq!(document.line_count(), 3);
    // assert_valid_line_number(&mut document);
}
