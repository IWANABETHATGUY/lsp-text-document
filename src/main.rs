use std::{fs::read_to_string, time::Instant};

use lsp_text_document::{compute_line_offsets, event, position, range, range_at, FullTextDocument};
use lsp_types::{Position, Url};

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
fn main() {
    // let mut document = new_document("我的你的真的加载\ntest");
    // let mut string = "我的你的".to_string();
    // println!("{:?}", string.chars().take(1).chain(string.chars().skip(3)).to_owned().collect::<String>());
    // // document.update(vec![event!("abc123", range!(3, 0, 6, 10))], 2);
    // println!("{:?}", document.get_line_offsets());
    // assert_eq!(document.text, "foo\nbarabc123");
    // assert_eq!(document.version, 2);
    // assert_eq!(document.offset_at(position!(1, 100)), 13);

    // assert_eq!(document.line_count(), 3);
    // assert_valid_line_number(&mut document);
    let string = read_to_string("test.js").unwrap();
    let start = Instant::now();
    for i in 0..100 {
        let text = string.clone();
        let _another = text[0..3].to_string() + "what" + &text[4..];
    }
    println!("{:?}", start.elapsed());

    let start = Instant::now();
    for i in 0..100 {
        let text = string.clone();
        let _another = text
            .chars()
            .take(3)
            .chain("what".chars())
            .chain(text.chars().skip(4))
            .collect::<String>();
    }
    println!("{:?}", start.elapsed());
    // self.text =
    //                 self.text.chars().take(start_offset).chain(change.text.chars()).chain(self.text.chars().skip(end_offset)).collect::<String>();
}
