use lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};

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
}

impl FullTextDocument {
    pub fn new(uri: Url, language_id: String, version: i64, text: String) -> FullTextDocument {
        // let item = lsp_types::TextDocumentItem::new(uri, language_id, version, text);
        FullTextDocument {
            uri,
            language_id,
            version,
            text,
            line_offset: None,
        }
    }

    pub fn update(&mut self, changes: Vec<TextDocumentContentChangeEvent>, version: i64) {
        for change in changes {
            if Self::is_incremental(&change) {
                // makes sure start is before end
                let range = get_wellformed_range(change.range.unwrap());

                let start_offset = self.offset_at(range.start);
                let end_offset = self.offset_at(range.end);

                self.text =
                    self.text[..start_offset].to_string() + &change.text + &self.text[end_offset..];

                let start_line = range.start.line as usize;
                let end_line = range.end.line as usize;
                let line_offsets = self.get_line_offsets();

                let mut add_line_offsets =
                    compute_line_offsets(&change.text, false, Some(start_offset));
                
                let add_line_offsets_len = add_line_offsets.len();
                if line_offsets.len() <= end_line as usize {
                    line_offsets.extend(vec![0; end_line as usize + 1 - line_offsets.len()]);
                }

                if end_line - start_line == add_line_offsets.len() {
                    for (i, offset) in add_line_offsets.into_iter().enumerate() {
                        line_offsets[i + start_line + 1] = offset;
                    }
                } else {
                    *line_offsets = {
                        let mut res = line_offsets[0..=start_line].to_vec();
                        res.append(&mut add_line_offsets);
                        res.extend_from_slice(&line_offsets[end_line + 1..]);
                        res
                    };
                }
                let diff = change.text.len() - (end_offset - start_offset);
                if diff != 0 {
                    for i in start_line + 1 + add_line_offsets_len..line_offsets.len() {
                        line_offsets[i] = line_offsets[i] + diff;
                    }
                }
            } else {
                self.text = change.text;
                self.line_offset = None;
            }
            self.version = version;
        }
    }

    // TODO:
    // public positionAt(offset: number): Position {
    // 	offset = Math.max(Math.min(offset, this._content.length), 0);

    // 	let lineOffsets = this.getLineOffsets();
    // 	let low = 0, high = lineOffsets.length;
    // 	if (high === 0) {
    // 		return { line: 0, character: offset };
    // 	}
    // 	while (low < high) {
    // 		let mid = Math.floor((low + high) / 2);
    // 		if (lineOffsets[mid] > offset) {
    // 			high = mid;
    // 		} else {
    // 			low = mid + 1;
    // 		}
    // 	}
    // 	// low is the least x for which the line offset is larger than the current offset
    // 	// or array.length if no line offset is larger than the current offset
    // 	let line = low - 1;
    // 	return { line, character: offset - lineOffsets[line] };
    // }
    // TODO:
    // public get lineCount() {
    // 		return this.getLineOffsets().length;
    // 	}
    pub fn is_incremental(event: &TextDocumentContentChangeEvent) -> bool {
        event.range_length.is_some() && event.range.is_some()
    }

    pub fn is_full(event: &TextDocumentContentChangeEvent) -> bool {
        !event.range_length.is_some() || !event.range.is_some()
    }

    fn get_line_offsets(&mut self) -> &mut Vec<usize> {
        if self.line_offset.is_none() {
            self.line_offset = Some(compute_line_offsets(&self.text, true, None));
        }
        self.line_offset.as_mut().unwrap()
    }
    fn offset_at(&mut self, position: Position) -> usize {
        let line_offsets = self.get_line_offsets();
        if position.line >= line_offsets.len() as u64 {
            return self.text.len();
        }
        let line_offset = line_offsets[position.line as usize];
        let next_line_offset = if position.line + 1 < line_offsets.len() as u64 {
            line_offsets[position.line as usize + 1]
        } else {
            self.text.len()
        };
        (line_offset + position.character as usize)
            .min(next_line_offset)
            .max(line_offset)
        // return Math.max(
        //     Math.min(line_offset + position.character, next_line_offset),
        //     line_offset,
        // );
    }
}

fn compute_line_offsets(
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
    for mut i in 0..char_array.len() {
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
