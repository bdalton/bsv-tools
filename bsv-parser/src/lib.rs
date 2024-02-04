//! A hand-written, fault-tolerant recursive descent parser for Bluespec SystemVerilog
//!

mod scanner;

pub use scanner::{CounterMode, Location, Position, Scanner, Span};
