//! Support for scanning multi-line UTF-8 strings with one token lookahead.
//!
//! There are two principle abstractions provided by this module.
//! The Scanner struct iterates over a UTF-8 string and the Position
//! struct keeps track of where in the string the current character
//! is at.
//!
//! There is a caveat. We would like our input string to be UTF-8
//! formatted and our position to use indices for either UTF-8,
//! UTF-16 or UTF-32. This is because we may be communicating
//! diagnostics different audiences. The Language Server protocol
//! prior to 2.17 only accepts UTF-16 character indices.
//!
//! We also want to keep track of all line UTF-8 (byte) offsets
//! which have been reached this far without making cloning a Scanner
//! an expensive operation.

mod counter_mode;
mod location;
mod position;
mod scanner;
mod span;

pub use counter_mode::CounterMode;
pub use location::Location;
pub use position::Position;
pub use scanner::Scanner;
pub use span::Span;
