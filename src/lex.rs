//! Lexer for Python Source Code
//!
//! Proivdes functions for converting python source into a series of logical
//! lines consisting of tokens.
//!
//! Based on https://docs.python.org/3.9/reference/lexical_analysis.html

/// A representation of a source block
struct Lines {
    start: u32,
    end: u32,
}

/// A tokenized value
trait Token {
    /// The line number(s) from which the token was parsed
    fn line_no(&self) -> u32;
}
