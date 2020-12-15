use lsp_text_document::{compute_line_offsets, event, position, range, range_at, FullTextDocument};
use lsp_types::{Position, Url};

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
fn main() {
    // let mut document = new_document("我的你的真的加载\ntest");
    // let mut string = "我的你的".to_string();
    // println!(
    //     "{:?}",
    //     string
    //         .chars()
    //         .take(1)
    //         .chain(string.chars().skip(3))
    //         .to_owned()
    //         .collect::<String>()
    // );
    // // document.update(vec![event!("abc123", range!(3, 0, 6, 10))], 2);
    // println!("{:?}", document.get_line_offsets());
    let mut document = new_document("foooo\nbaz\n");
    assert_eq!(document.offset_at(position!(1, 0)), 6);
    document.update(vec![event!("//我的你的", range!(1, 3, 1, 3))], 1);
    document.update(vec![event!("let a = 3", range!(2, 0, 2, 0))], 2);
    println!("{:?}", document.text);
    // assert_eq!(document.text, "foooo\n我的你的\nbar some extra content\nbaz");
    // assert_eq!(document.version, 1);
    // println!("{}", "我的你的".len())
    // assert_eq!(document.offset_at(position!(2, 0)), 29);
    // assert_valid_line_number(&mut document);
    // assert_eq!(document.text, "foo\nbarabc123");
    // assert_eq!(document.version, 2);
    // assert_eq!(document.offset_at(position!(1, 100)), 13);

    // assert_eq!(document.line_count(), 3);
    // assert_valid_line_number(&mut document);
}
