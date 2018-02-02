/// Copyright (c) Peter Sanders. All rights reserved.
/// Date: 2018-02-01
extern crate num;
extern crate regex;

use std::io::BufRead;
use std::str;
use std::str::FromStr;

use regex::Regex; // For regex "delim"
use num::Integer;
use num::Float;

#[cfg(test)]
mod tests;

/// Rust implementation of java.util.Scanner
pub struct Scanner<'a> {
    stream: &'a mut BufRead, // Underlying stream object we are handling
    delim: Regex,            // Delimiter used to specify word boundaries
}

/// Implements the meta-methods of Scanner that affect how the data stream
/// is processed, e.g., delimiter, parsing radix, etc.
impl<'a> Scanner<'a> {
    /// Sets the delimiter to be some pre-compiled regex.
    pub fn set_delim(&mut self, delim: Regex) -> &Regex {
        self.delim = delim;

        &self.delim
    }

    /// Sets the delimiter to be a string literal. The resulting delimiting
    /// expression is guaranteed to only interpret the literal passed in,
    /// i.e., this method cannot be used to simultaneously compile and set
    /// an arbitrary regular expression.
    pub fn set_delim_str(&mut self, delim: &str) -> &Regex {
        // We escape any regex metacharacters, so the result is a
        // string literal that is guaranteed to be a safe regex.
        self.delim = Regex::new(regex::escape(delim).as_str()).unwrap();

        &self.delim
    }

    /// Return the delimiter for `Scanner.next()`
    /// and methods that depend on it.
    pub fn get_delim(&self) -> &Regex {
        &self.delim
    }
}

/// Implements the methods of Scanner that affect the underlying data stream
impl<'a> Scanner<'a> {
    /// Creates a new instance of Scanner on some object implementing `BufRead`
    pub fn new(stream: &'a mut BufRead) -> Scanner {
        Scanner {
            stream: stream,
            // We can safely unwrap this regex because it is hard-coded.
            delim: Regex::new(r"\s+").unwrap(),
        }
    }

    /// Returns Some(String) containing the next string if there is one.
    /// Otherwise returns None.
    ///
    /// We first consume all leading `delim`s, then attempt to read everything
    /// until (but excluding) the next `delim`. If this results in an empty
    /// string, we will return `None`.
    pub fn next(&mut self) -> Option<String> {
        let mut consume_counter = 0;
        let mut res = String::new();

        consume_counter = {
            if let Ok(buf) = self.stream.fill_buf() {
                // If the buffer is not a valid utf-8 string, we exit the
                // method with `None` result.
                if str::from_utf8(buf).is_err() {
                    return None;
                }

                // The check above guarantees `unwrap` will succeed.
                let mut input: &str = str::from_utf8(buf).unwrap();

                // While the front of the buffer matches `delim`, skip it.
                while let Some(found) = self.delim.find(input) {
                    if found.start() > 0 {
                        break;
                    }
                    consume_counter += found.end();
                    input = &input[found.end()..];
                }

                if let Some(found) = self.delim.find(input) {
                    res = String::from(&input[..found.start()]);

                    consume_counter + found.start()
                } else {
                    res = String::from(input);

                    consume_counter + input.len()
                }
            } else {
                0
            }
        };
        self.stream.consume(consume_counter);

        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }

    /// Read up to (but excluding) the next `\n` character.
    /// If there are any leading `delim`s, they will be included in the
    /// returned string.
    ///
    /// NOTE: unlike `next()` we do consume the trailing `\n`, if it exists.
    pub fn next_line(&mut self) -> Option<String> {
        let mut res = String::new();

        if let Ok(_size) = self.stream.read_line(&mut res) {
            if let Some(end) = res.pop() {
                if end == '\n' {
                    Some(res)
                } else {
                    res.push(end);

                    Some(res)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Attempts to retrieve the next integer of the specified (or inferred)
    /// type. Even if this fails, we still consume `next`.
    pub fn next_int<T: Integer + FromStr>(&mut self) -> Option<T> {
        if let Some(mut input) = self.next() {
            // Strip commas. Numbers with commas are considered valid
            // but Rust does not recognize them in its default behavior.
            while let Some(comma_idx) = input.rfind(',') {
                input.remove(comma_idx);
            }

            match input.parse::<T>() {
                Ok(res) => Some(res),
                Err(_e) => None,
            }
        } else {
            None
        }
    }

    /// Attempts to retrieve the next floating-point number of the specified
    /// (or inferred) type. Even if this fails, we still consume `next`.
    ///
    /// Note that this method is based on `Scanner.next()`, so the delimiter
    /// is still the same.
    pub fn next_float<T: Float + FromStr>(&mut self) -> Option<T> {
        if let Some(mut input) = self.next() {
            // Strip commas. Numbers with commas are considered valid
            // but Rust does not recognize them in its default behavior.
            while let Some(comma_idx) = input.rfind(',') {
                input.remove(comma_idx);
            }

            match input.parse::<T>() {
                Ok(res) => Some(res),
                Err(_e) => None,
            }
        } else {
            None
        }
    }

}
