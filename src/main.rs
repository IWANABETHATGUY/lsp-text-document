use std::{fs::read_to_string, time::Instant};

use lsp_text_document::FullTextDocument;
use lsp_types::Url;

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
    let s = 10000;
    let e = 10100;
    let string = read_to_string("test.vue").unwrap();
    let start = Instant::now();
    let mut first_another = "".to_string();

    for _ in 0..100 {
        let text = string.clone();
        let start_byte = string
            .chars()
            .take(s)
            .fold(0, |acc, cur| acc + cur.len_utf8());
        let end_byte = string
            .chars()
            .skip(s)
            .take(e - s)
            .fold(0, |acc, cur| acc + cur.len_utf8())
            + start_byte;
        first_another = text[0..start_byte].to_string() + "what" + &text[end_byte..];
        // println!("{:?}", _another);
    }
    println!("{:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..100 {
        let text = string.clone();
        let _another = text
            .chars()
            .take(s)
            .chain("what".chars())
            .chain(text.chars().skip(e))
            .collect::<String>();
        assert!(&first_another == &_another);
    }
    println!("{:?}", start.elapsed());
    // self.text =
    //                 self.text.chars().take(start_offset).chain(change.text.chars()).chain(self.text.chars().skip(end_offset)).collect::<String>();
}
