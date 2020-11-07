use lsp_text_document::compute_line_offsets;

fn main() {
    let res = compute_line_offsets(&"ABCDE\r\nFGHIJ".into(), true, None);
    println!("{:?}", res);

}
