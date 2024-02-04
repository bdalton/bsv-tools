use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.character + 1)
    }
}

impl Position {
    pub const START: Position = Position {
        line: 0,
        character: 0,
    };

    pub fn bump_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }

    pub fn bump_character(&mut self, n: u32) {
        self.character += n;
    }
}
