use crate::{CounterMode, Position};
use std::sync::Arc;
use std::{fmt, str};
use url::Url;

pub struct Scanner<'a> {
    source: str::Chars<'a>,
    pub uri: Arc<Url>,
    pub lookahead: Option<char>,
    pub pos: Position,
    mode: CounterMode,
}

impl<'a> Scanner<'a> {
    pub fn new(uri: Arc<Url>, text: &'a str, mode: CounterMode) -> Scanner<'a> {
        let mut source = text.chars();
        let lookahead = source.next();

        Scanner {
            source,
            uri,
            lookahead,
            pos: Position::START,
            mode,
        }
    }

    // advance the scanner to the next character
    //
    // This updates the lookahead and position of the scanner.
    pub fn advance(&mut self) {
        if let Some(prev_ch) = self.lookahead.take() {
            self.lookahead = self.source.next();
            if prev_ch == '\n' {
                self.pos.bump_line();
            } else if prev_ch == '\r' && self.lookahead == Some('\n') {
                self.lookahead = self.source.next();
                self.pos.bump_line();
            } else {
                let n = self.mode.increment_for(prev_ch);
                self.pos.bump_character(n);
            }
        }
    }
}

impl<'a> fmt::Debug for Scanner<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scanner")
            .field("position", &self.pos)
            .finish()
    }
}
