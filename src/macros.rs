#[macro_export]
macro_rules! position {
    ($line:expr, $character:expr) => {{
        Position::new($line as u64, $character as u64)
    }};
}
