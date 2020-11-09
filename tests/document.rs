use lsp_text_document::{range_at, FullTextDocument};
use lsp_types::{TextDocumentContentChangeEvent, Url};

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
enum InsertPosition {
    After,
    At,
}

#[cfg(test)]
mod test_document_model {
    use super::*;
    use lsp_text_document::position;
    use lsp_types::Position;
    #[test]
    fn test_empty_content() {
        let mut document = new_document("");
        assert_eq!(document.line_count(), 1);
        assert_eq!(document.offset_at(position!(0, 0)), 0);
        assert_eq!(document.position_at(0), position!(0, 0));
    }

    #[test]
    fn test_single_line() {
        let str = "Hello world";
        let mut document = new_document(str);
        assert_eq!(document.line_count(), 1);
        for i in 0..str.len() {
            assert_eq!(document.offset_at(position!(0, i)), i);
            assert_eq!(document.position_at(i as u64), position!(0, i));
        }
    }

    #[test]
    fn test_multi_line() {
        let str = "ABCDE\nFGHIJ\nKLMNO\n";
        let mut document = new_document(str);
        assert_eq!(document.line_count(), 4);
        for i in 0..str.len() {
            let line = i / 6;
            let column = i % 6;
            assert_eq!(document.offset_at(position!(line, column)), i);
            assert_eq!(document.position_at(i as u64), position!(line, column));
        }
        assert_eq!(document.offset_at(position!(3, 0)), 18);
        assert_eq!(document.offset_at(position!(3, 1)), 18);
        assert_eq!(document.position_at(18u64), position!(3, 0));
        assert_eq!(document.position_at(19u64), position!(3, 0));
    }

    #[test]
    fn test_starts_with_new_line() {
        let str = "\nABCDE";
        let mut document = new_document(str);
        assert_eq!(document.line_count(), 2);
        assert_eq!(document.offset_at(position!(0, 0)), 0);
        assert_eq!(document.offset_at(position!(1, 1)), 2);

        assert_eq!(document.position_at(0), position!(0, 0));
        assert_eq!(document.position_at(1), position!(1, 0));
        assert_eq!(document.position_at(6), position!(1, 5));
    }

    #[test]
    fn test_new_line_character() {
        let mut str = "ABCDE\rFGHIJ";
        assert_eq!(new_document(str).line_count(), 2);
        str = "ABCDE\nFGHIJ";
        assert_eq!(new_document(str).line_count(), 2);

        str = "ABCDE\r\nFGHIJ";
        assert_eq!(new_document(str).line_count(), 2);

        str = "ABCDE\n\nFGHIJ";
        assert_eq!(new_document(str).line_count(), 3);

        str = "ABCDE\r\rFGHIJ";
        assert_eq!(new_document(str).line_count(), 3);

        str = "ABCDE\n\rFGHIJ";
        assert_eq!(new_document(str).line_count(), 3);
    }
    #[ignore]
    #[test]
    fn test_text_range() {
        // TODO:
    }

    #[test]
    fn test_invalid_input() {
        let str = "Hello World";
        let mut document = new_document(str);

        // invalid position
        assert_eq!(document.offset_at(position!(0, str.len())), str.len());
        assert_eq!(document.offset_at(position!(0, str.len() + 3)), str.len());
        assert_eq!(document.offset_at(position!(2, 3)), str.len());

        // invalid offsets
        assert_eq!(
            document.position_at(str.len() as u64),
            position!(0, str.len())
        );
        assert_eq!(
            document.position_at(str.len() as u64 + 3),
            position!(0, str.len())
        );
    }
}

#[cfg(test)]
mod test_document_full_update {
    use lsp_types::TextDocumentContentChangeEvent;

    use super::*;

    #[test]
    fn test_one_full_update() {
        let mut document = new_document("abc123");
        document.update(
            vec![TextDocumentContentChangeEvent {
                text: "efg456".into(),
                range: None,
                range_length: None,
            }],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(document.text, "efg456");
    }

    #[test]
    fn test_several_full_update() {
        let mut document = new_document("abc123");
        document.update(
            vec![
                TextDocumentContentChangeEvent {
                    text: "hello".into(),
                    range: None,
                    range_length: None,
                },
                TextDocumentContentChangeEvent {
                    text: "world".into(),
                    range: None,
                    range_length: None,
                },
            ],
            2,
        );
        assert_eq!(document.version, 2);
        assert_eq!(document.text, "world");
    }
}

#[cfg(test)]
mod test_document_incremental_update {
    use super::*;
    use lsp_text_document::{ie, range_after, range_at, re};
    use lsp_types::{Range, TextDocumentContentChangeEvent};
    fn assert_valid_line_number(doc: &mut lsp_text_document::FullTextDocument) {
        let text = doc.text.to_string();
        let mut expected_line_number = 0;
        for i in 0..text.len() {
            assert_eq!(doc.position_at(i as u64).line, expected_line_number);
            let ch = text.chars().nth(i).unwrap();
            if ch == '\n' {
                expected_line_number += 1;
            }
        }
        assert_eq!(
            doc.position_at(text.len() as u64).line,
            expected_line_number
        );
    }
    #[test]
    fn test_removing_content() {
        let mut document = new_document("function abc() {\n  console.log(\"hello, world!\");\n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
        document.update(
            vec![TextDocumentContentChangeEvent {
                text: "".into(),
                range: Some(range_at!(document.clone(), "hello, world!")),
                range_length: Some(0),
            }],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(document.text, "function abc() {\n  console.log(\"\");\n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_remove_multi_line_content() {
        let mut document = new_document("function abc() {\n  foo();\n  bar();\n  \n}");
        assert_eq!(document.line_count(), 5);
        assert_valid_line_number(&mut document);
        document.update(
            vec![TextDocumentContentChangeEvent {
                text: "".into(),
                range: Some(range_at!(document.clone(), "  foo();\n  bar();\n")),
                range_length: Some(0),
            }],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(document.text, "function abc() {\n  \n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_remove_multi_line_content2() {
        let mut document = new_document("function abc() {\n  foo();\n  bar();\n  \n}");
        assert_eq!(document.line_count(), 5);
        assert_valid_line_number(&mut document);
        document.update(
            vec![TextDocumentContentChangeEvent {
                text: "".into(),
                range: Some(range_at!(document.clone(), "foo();\n  bar();")),
                range_length: Some(0),
            }],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(document.text, "function abc() {\n  \n  \n}");
        assert_eq!(document.line_count(), 4);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_add_content() {
        let mut document = new_document("function abc() {\n  console.log(\"hello\");\n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
        document.update(vec![ie!(", world!", document.clone(), "hello")], 1);
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "function abc() {\n  console.log(\"hello, world!\");\n}"
        );
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_add_multi_line_content() {
        let mut document =
            new_document("function abc() {\r\n  while (true) {\n    foo();\n  };\n}");
        assert_eq!(document.line_count(), 5);
        assert_valid_line_number(&mut document);
        document.update(vec![ie!("\n    bar();", document.clone(), "foo();")], 1);
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "function abc() {\r\n  while (true) {\n    foo();\n    bar();\n  };\n}"
        );
        assert_eq!(document.line_count(), 6);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_replace_single_line_content() {
        let mut document = new_document("function abc() {\n  console.log(\"hello, world!\");\n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
        document.update(
            vec![re!("world, hello!", document.clone(), "hello, world!")],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "function abc() {\n  console.log(\"world, hello!\");\n}"
        );
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
    }

    #[test]
    fn test_replace_multi_line_content() {
        let mut document = new_document("function abc() {\n  console.log(\"hello, world!\");\n}");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
        document.update(
            vec![re!(
                "\n//hello\nfunction d(){",
                document.clone(),
                "function abc() {"
            )],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "\n//hello\nfunction d(){\n  console.log(\"hello, world!\");\n}"
        );
        assert_eq!(document.line_count(), 5);
        assert_valid_line_number(&mut document);
    }
    #[test]
    fn test_replace_multi_line_content_less_line() {
        let mut document = new_document("a1\nb1\na2\nb2\na3\nb3\na4\nb4\n");
        assert_eq!(document.line_count(), 9);
        assert_valid_line_number(&mut document);
        document.update(
            vec![re!(
                "xx\nyy",
                document.clone(),
                "\na3\nb3\na4\nb4\n"
            )],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "a1\nb1\na2\nb2xx\nyy"
        );
        assert_eq!(document.line_count(), 5);
        assert_valid_line_number(&mut document);
    }


    #[test]
    /// Incrementally replacing multi-line content, same num of lines and chars
    fn test_replace_multi_line_content_less_line2() {
        let mut document = new_document("a1\nb1\na2\nb2\na3\nb3\na4\nb4\n");
        assert_eq!(document.line_count(), 9);
        assert_valid_line_number(&mut document);
        document.update(
            vec![re!(
                "\nxx1\nxx2",
                document.clone(),
                "a2\nb2\na3"
            )],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "a1\nb1\n\nxx1\nxx2\nb3\na4\nb4\n"
        );
        assert_eq!(document.line_count(), 9);
        assert_valid_line_number(&mut document);
    }


    #[test]
    /// Incrementally replacing multi-line content, same num of lines but diff chars
    fn test_replace_multi_line_content_less_line3() {
        let mut document = new_document("a1\nb1\na2\nb2\na3\nb3\na4\nb4\n");
        assert_eq!(document.line_count(), 9);
        assert_valid_line_number(&mut document);
        document.update(
            vec![re!(
                "\ny\n",
                document.clone(),
                "a2\nb2\na3"
            )],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "a1\nb1\n\ny\n\nb3\na4\nb4\n"
        );
        assert_eq!(document.line_count(), 9);
        assert_valid_line_number(&mut document);
    }

    #[test]
    /// Incrementally replacing multi-line content, huge number of lines
    fn test_replace_multi_line_content_less_line4() {
        let mut document = new_document("a1\ncc\nb1");
        assert_eq!(document.line_count(), 3);
        assert_valid_line_number(&mut document);
        let text: String = String::from("dd") + &"\ndd".repeat(1999);
        document.update(
            vec![re!(
                &text,
                document.clone(),
                "\ncc"
            )],
            1,
        );
        assert_eq!(document.version, 1);
        assert_eq!(
            document.text,
            "a1".to_string() + &text + "\nb1"
        );
        assert_eq!(document.line_count(), 2001);
        assert_valid_line_number(&mut document);
    }

    // #[test]
    // fn test_
}
