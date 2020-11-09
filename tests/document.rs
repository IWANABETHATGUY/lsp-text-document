use lsp_text_document::FullTextDocument;
use lsp_types::Url;

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
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
