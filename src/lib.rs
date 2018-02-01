/// Copyright (c) Peter Sanders. All rights reserved.
/// Date: 2018-01-30

use std::io::BufRead;
use std::vec::Vec;
//use regex::Regex;  // For regex "delim"

/// Rust implementation of java.util.Scanner
pub struct Scanner<'a> {
    stream: &'a mut BufRead, // Underlying stream object we are handling
    delim: char,             // Delimiter used to specify word boundaries
}

/// Implements the next* methods.
impl<'a> Scanner<'a> {
    /// Creates a new instance of Scanner
    pub fn new(stream: &'a mut BufRead) -> Scanner {
        Scanner {
            stream: stream,
            delim: ' ',
        }
    }

    /// Returns Some(String) containing the next string if there is one.
    /// Otherwise returns None.
    ///
    /// We first consume all leading `delim`s, then attempt to read everything
    /// until (but excluding) the next `delim`. If this results in an empty
    /// string, we will return `None`.
    pub fn next(&mut self) -> Option<String> {
        let mut buf: Vec<u8> = Vec::new();

        if let Ok(length) = self.stream.read_until(self.delim as u8, &mut buf) {
            // Skip leading `delim` characters
            if buf[0] == self.delim as u8 {
                return self.next();
            }

            // Remove trailing `delim` character if it exists.
            // NOTE: we will have one trailing `delim` unless we read to EOF.
            if buf[buf.len() - 1] == self.delim as u8 {
                buf.pop();
            }

            if let Ok(res) = String::from_utf8(buf) {
                Some(res)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn next_line(&mut self) -> Option<String> {
        let old_delim = self.delim;
        self.delim = '\n';

        let res = self.next();
        self.delim = old_delim;

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn next_works_once_when_good_input() {
        let mut string: &[u8] = b"hello";
        let mut test: Scanner = Scanner::new(&mut string);
        if let Some(res) = test.next() {
            assert_eq!(&res[..], "hello");
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn next_breaks_at_char_delim() {
        let mut string: &[u8] = b"hello, world";
        let mut test: Scanner = Scanner::new(&mut string);
        if let Some(res) = test.next() {
            assert_eq!(&res[..], "hello,");
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn next_skips_leading_delims() {
        let mut string: &[u8] = b"hello,  world";
        let mut test: Scanner = Scanner::new(&mut string);
        test.next();
        if let Some(res) = test.next() {
            assert_eq!(&res[..], "world");
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn next_line_reads_whole_line() {
        let mut string: &[u8] = b"hello,  world\ngoodbye, world";
        let mut test: Scanner = Scanner::new(&mut string);
        if let Some(res) = test.next_line() {
            assert_eq!(&res[..], "hello,  world");
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn next_works_after_next_line() {
        let mut string: &[u8] = b"hello,  world\ngoodbye, world";
        let mut test: Scanner = Scanner::new(&mut string);
        test.next_line();
        if let Some(res) = test.next() {
            assert_eq!(&res[..], "goodbye,");
        } else {
            assert_eq!(true, false);
        }
    }
}
