use lsp_types::Range;

#[macro_export]
macro_rules! position {
    ($line:expr, $character:expr) => {{
        Position::new($line as u64, $character as u64)
    }};
}

#[macro_export]
macro_rules! range_at {
    ($doc:expr, $sub:expr) => {{
        let index = $doc.text.find($sub).unwrap();
        lsp_types::Range::new(
            $doc.position_at(index as u64),
            $doc.position_at(index as u64 + $sub.len() as u64),
        )
    }};
}

#[macro_export]
macro_rules! range_after {
    ($doc:expr, $sub:expr ) => {{
        let index = $doc.text.find($sub).unwrap() + $sub.len();
        lsp_types::Range::new(
            $doc.position_at(index as u64),
            $doc.position_at(index as u64),
        )
    }};
}
#[macro_export]
/// an insert TextDocumentContentChangeEvent
macro_rules! ie {
    ($text:expr, $doc:expr, $sub_str:expr) => {
        {
            TextDocumentContentChangeEvent {
                text: $text.into(),
                range: Some(range_after!($doc, $sub_str)),
                range_length: None,
            }
        }
    };
}
#[macro_export]
macro_rules! re {
    ($text:expr, $doc:expr, $sub_str:expr) => {
        {
            TextDocumentContentChangeEvent {
                text: $text.into(),
                range: Some(range_at!($doc, $sub_str)),
                range_length: None,
            }
        }
    };
}