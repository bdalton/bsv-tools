// A `Position` counts a character offset from the beginning of the
// line. Depending on the use-case this character offset may need
// to keep track of the character for string encodings. For example,
// the Language Server Protocol typically needs character positions
// to within a UTF-16 string. The CounterMode allows us to select
// the semantics of the `character` field for all `Position`s generated
// by a `Scanner`.
pub enum CounterMode {
    Utf8,
    Utf16,
    Utf32,
}

impl CounterMode {
    #[inline]
    pub fn increment_for(&self, ch: char) -> u32 {
        if ch < '\u{80}' {
            // optimize for common case
            // ASCII characters increment character by 1 in all modes
            1
        } else {
            match self {
                CounterMode::Utf8 => {
                    // valid UTF-8 can be encoded in upto 4 u8s.
                    if ch < '\u{800}' {
                        2
                    } else if ch < '\u{10000}' {
                        3
                    } else {
                        4
                    }
                },
                CounterMode::Utf16 => {
                    // valid UTF-16 can be encoded in 1 or 2 u16s.
                    if ch < '\u{10000}' {
                        1
                    } else {
                        2
                    }
                },
                CounterMode::Utf32 => {
                    // All codepoints fit within a single u32.
                    1
                }
            }
        }
    }
}
