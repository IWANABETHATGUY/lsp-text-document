use lsp_text_document::FullTextDocument;
use lsp_types::Url;

pub fn new_document(str: &str) -> FullTextDocument {
    let url = Url::parse("file://foo");
    FullTextDocument::new(url.unwrap(), "text".into(), 0, str.to_string())
}
#[cfg(test)]
mod test_document_model {
    use lsp_types::Position;

    use super::*;

    #[test]
    fn test_empty_content() {
        let mut document = new_document("");
        assert_eq!(document.line_count(), 1);
        assert_eq!(document.offset_at(Position::new(0, 0)), 0);
    }

    #[test]
    fn test_single_line() {
        let str = "Hello world";
        let mut document = new_document(str);
        assert_eq!(document.line_count(), 1);
        for i in 0..str.len() {
            assert_eq!(document.offset_at(Position::new(0, i as u64)), i);
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
            assert_eq!(
                document.offset_at(Position::new(line as u64, column as u64)),
                i
            );
        }
        assert_eq!(document.offset_at(Position::new(3 as u64, 0 as u64)), 18);
        assert_eq!(document.offset_at(Position::new(3 as u64, 1 as u64)), 18);
    }

    #[test]
    fn test_starts_with_new_line() {
        let str = "\nABCDE";
        let mut document = new_document(str);
        assert_eq!(document.line_count(), 2);
        assert_eq!(document.offset_at(Position::new(0 as u64, 0 as u64)), 0);
        assert_eq!(document.offset_at(Position::new(1 as u64, 1 as u64)), 2);
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
}
