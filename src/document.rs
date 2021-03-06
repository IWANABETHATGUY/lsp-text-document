use lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};
use ropey::Rope;
#[derive(Clone)]
pub struct FullTextDocument {
    pub uri: Url,

    /// The text document's language identifier.
    pub language_id: String,

    /// The version number of this document (it will strictly increase after each
    /// change, including undo/redo).
    pub version: i64,

    /// The content of the opened text document.
    pub text: String,

    line_offset: Option<Vec<usize>>,
    pub rope: Rope,
}

impl FullTextDocument {
    pub fn new(uri: Url, language_id: String, version: i64, text: String) -> FullTextDocument {
        // let item = lsp_types::TextDocumentItem::new(uri, language_id, version, text);
        FullTextDocument {
            uri,
            language_id,
            version,
            text: text.clone(),
            line_offset: None,
            rope: Rope::from_str(&text),
        }
    }

    pub fn update(&mut self, changes: Vec<TextDocumentContentChangeEvent>, version: i64) {
        for change in changes {
            if Self::is_incremental(&change) {
                // makes sure start is before end
                let range = get_wellformed_range(change.range.unwrap());

                let start_offset = self.offset_at(range.start);
                let end_offset = self.offset_at(range.end);

                // let (start_byte, end_byte) =
                //     self.transform_offset_to_byte_offset(start_offset, end_offset);
                self.rope.remove(start_offset..end_offset);
                self.rope.insert(start_offset, &change.text);
                // self.text =
                //     self.text.chars().take(start_offset).chain(change.text.chars()).chain(self.text.chars().skip(end_offset)).collect::<String>();
                // let start_line = range.start.line as usize;
                // let end_line = range.end.line as usize;
                // let line_offsets = self.get_line_offsets();

                // let mut add_line_offsets =
                //     compute_line_offsets(&change.text, false, Some(start_offset));

                // let add_line_offsets_len = add_line_offsets.len();
                // // if line_offsets.len() <= end_line as usize {
                // //     line_offsets.extend(vec![0; end_line as usize + 1 - line_offsets.len()]);
                // // }

                // if end_line - start_line == add_line_offsets.len() {
                //     for (i, offset) in add_line_offsets.into_iter().enumerate() {
                //         line_offsets[i + start_line + 1] = offset;
                //     }
                // } else {
                //     *line_offsets = {
                //         let mut res =
                //             line_offsets[0..=start_line.min(line_offsets.len() - 1)].to_vec();
                //         res.append(&mut add_line_offsets);
                //         res.extend_from_slice(
                //             &line_offsets[end_line.min(line_offsets.len() - 1) + 1..],
                //         );
                //         res
                //     };
                // }
                // let diff: i32 = change.text.len() as i32 - (end_offset - start_offset) as i32;
                // if diff != 0 {
                //     for i in start_line + 1 + add_line_offsets_len..line_offsets.len() {
                //         line_offsets[i] = (line_offsets[i] as i32 + diff) as usize;
                //     }
                // }
            } else if Self::is_full(&change) {
                self.text = change.text;
                self.line_offset = None;
            }
            self.version = version;
        }
    }

    pub fn transform_offset_to_byte_offset(
        &self,
        start_offset: usize,
        end_offset: usize,
    ) -> (usize, usize) {
        let start_byte = self
            .text
            .chars()
            .take(start_offset)
            .fold(0, |acc, cur| acc + cur.len_utf8());
        let end_byte = self
            .text
            .chars()
            .skip(start_offset)
            .take(end_offset - start_offset)
            .fold(0, |acc, cur| acc + cur.len_utf8())
            + start_byte;
        (start_byte, end_byte)
    }
    pub fn position_at(&mut self, mut offset: u32) -> Position {
        offset = offset.min(self.text.len() as u32).max(0);

        let line_offsets = self.get_line_offsets();
        // let low = 0, high = lineOffsets.length;
        let mut low = 0usize;
        let mut high = line_offsets.len();
        if high == 0 {
            return Position {
                line: 0,
                character: offset,
            };
        }
        while low < high {
            let mid = low + (high - low) / 2;
            if line_offsets[mid] as u32 > offset {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        let line = low as u32 - 1;
        return Position {
            line,
            character: offset - line_offsets[line as usize] as u32,
        };
    }

    pub fn line_count(&mut self) -> usize {
        self.rope.len_lines()
    }
    pub fn is_incremental(event: &TextDocumentContentChangeEvent) -> bool {
        event.range.is_some()
    }

    pub fn is_full(event: &TextDocumentContentChangeEvent) -> bool {
        !event.range_length.is_some() && !event.range.is_some()
    }

    pub fn get_line_offsets(&mut self) -> &mut Vec<usize> {
        if self.line_offset.is_none() {
            self.line_offset = Some(compute_line_offsets(&self.text, true, None));
        }
        self.line_offset.as_mut().unwrap()
    }

    pub fn get_text(&self) -> &str {
        let end_char = self.rope.len_chars();
        self.rope.slice(0..end_char).as_str().unwrap_or("")
    }
    pub fn offset_at(&mut self, position: Position) -> usize {
        let Position { line, character } = position;
        // line_to_char + character as usize
        // let line_offsets = self.get_line_offsets();
        if position.line >= self.line_count() as u32 {
            return self.rope.len_chars();
            // return self.text.chars().fold(0, |acc, cur| acc + cur.len_utf8());
        }
        let line_offset = self.rope.line_to_char(line as usize);
        let next_line_offset = if position.line + 1 < self.line_count() as u32 {
            self.rope.line_to_char(line as usize + 1)
        } else {
            self.rope.len_chars()
        };
        (line_offset + character as usize)
            .min(next_line_offset)
            .max(line_offset)
    }
}

pub fn compute_line_offsets(
    text: &String,
    is_at_line_start: bool,
    text_offset: Option<usize>,
) -> Vec<usize> {
    let text_offset = if let Some(offset) = text_offset {
        offset
    } else {
        0
    };
    let mut result = if is_at_line_start {
        vec![text_offset]
    } else {
        vec![]
    };
    let char_array: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < char_array.len() {
        let &ch = unsafe { char_array.get_unchecked(i) };
        if ch == '\r' || ch == '\n' {
            if ch == '\r'
                && i + 1 < char_array.len()
                && unsafe { char_array.get_unchecked(i + 1) == &'\n' }
            {
                i += 1;
            }
            result.push(text_offset + i + 1);
        }
        i += 1;
    }
    result
}

fn get_wellformed_range(range: Range) -> Range {
    let start = range.start;
    let end = range.end;
    if start.line > end.line || (start.line == end.line && start.character > end.character) {
        Range::new(end, start)
    } else {
        range
    }
}
