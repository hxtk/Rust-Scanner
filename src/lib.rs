/// Copyright (c) Peter Sanders. All rights reserved.
/// Date: 2018-02-02
extern crate num;
extern crate regex;

use std::io::BufRead;
use std::str;

use regex::Regex; // For regex "delim"
use num::Integer;
use num::Float;

#[cfg(test)]
mod tests;

/// Rust implementation of java.util.Scanner
pub struct Scanner<'a> {
    stream: &'a mut BufRead, // Underlying stream object we are handling
    delim: Regex,            // Delimiter used to specify word boundaries
    radix: u32,              // Base in which we parse numeric types
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

    /// Sets the radix in which numbers are parsed. This value must be on
    /// the closed range [2, 36], such that alphabet characters represent
    /// values greater than 9 in bases exceeding 10.
    pub fn set_radix(&mut self, radix: u32) -> u32 {
        if 1 < radix && radix <= 36 {
            self.radix = radix;
        }
        self.radix
    }

    pub fn get_radix(&self) -> u32 {
        self.radix
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
            radix: 10,
        }
    }

    /// Returns `Some(String)` containing the next string if there is one.
    /// Otherwise returns `None`.
    ///
    /// We first consume all leading `delim`s, then attempt to read everything
    /// until (but excluding) the next `delim`. If this results in an empty
    /// string, we will return `None`.
    pub fn next(&mut self) -> Option<String> {
        self.consume_leading_delims();

        let mut res = String::new();

        loop {
            let (length, end) = {
                if let Ok(buf) = self.stream.fill_buf() {
                    // If the buffer is not a valid utf-8 string, we exit the
                    // method with `None` result.
                    if str::from_utf8(buf).is_err() {
                        return None;
                    }
                    
                    // The check above guarantees `unwrap` will succeed.
                    let mut input: &str = str::from_utf8(buf).unwrap();
                    
                    if let Some(found) = self.delim.find(input) {
                        res.push_str(&input[..found.start()]);
                        
                        (found.start(), true)
                    } else {
                        res.push_str(input);
                        
                        (input.len(), false)
                    }
                } else {
                    (0, true)
                }
            };
            self.stream.consume(length);

            if end || length == 0 {
                break;
            }
        }

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
    ///
    /// The default radix for this parsing is 10, but users may specify a
    /// one-time arbitrary radix using `Scanner.next_int_radix(u32)`
    /// or persistently using `Scanner.set_radix(u32)`.
    pub fn next_int<T: Integer>(&mut self) -> Option<T> {
        if let Some(mut input) = self.next() {
            // Strip commas. Numbers with commas are considered valid
            // but Rust does not recognize them in its default behavior.
            while let Some(comma_idx) = input.rfind(',') {
                input.remove(comma_idx);
            }

            match <T>::from_str_radix(input.as_str(), self.radix) {
                Ok(res) => Some(res),
                Err(_e) => None,
            }
        } else {
            None
        }
    }

    /// Returns the next integer in some arbitrary base on [2, 36].
    ///
    /// If the radix provided is outside of this range, we do nothing.
    /// Otherwise, we will consume `next()` even if it is not a valid integer.
    ///
    /// NOTE: If one means to repeatedly parse in a fixed, arbitrary base,
    /// it is more efficient to use `Scanner.set_radix(u32)` followed by
    /// `Scanner.next_int` with no radix argument.
    pub fn next_int_radix<T: Integer>(&mut self, radix: u32) -> Option<T> {
        if radix < 2 || radix > 36 {
            None
        } else {
            let old_radix = self.radix;
            self.set_radix(radix);

            let res = self.next_int::<T>();
            self.set_radix(old_radix);

            res
        }
    }

    /// Attempts to retrieve the next floating-point number of the specified
    /// (or inferred) type. Even if this fails, we still consume `next`.
    ///
    /// Note that this method is based on `Scanner.next()`, so the delimiter
    /// is still the same.
    pub fn next_float<T: Float>(&mut self) -> Option<T> {
        if let Some(mut input) = self.next() {
            // Strip commas. Numbers with commas are considered valid
            // but Rust does not recognize them in its default behavior.
            while let Some(comma_idx) = input.rfind(',') {
                input.remove(comma_idx);
            }

            match <T>::from_str_radix(input.as_str(), self.radix) {
                Ok(res) => Some(res),
                Err(_e) => None,
            }
        } else {
            None
        }
    }

    /// Returns the next float in some arbitrary base on [2, 36].
    ///
    /// If the radix provided is outside of this range, we do nothing.
    /// Otherwise, we will consume `next()` even if it is not a valid integer.
    ///
    /// NOTE: If one means to repeatedly parse in a fixed, arbitrary base,
    /// it is more efficient to use `Scanner.set_radix(u32)` followed by
    /// `Scanner.next_float` with no radix argument.
    pub fn next_float_radix<T: Float>(&mut self, radix: u32) -> Option<T> {
        if radix < 2 || radix > 36 {
            None
        } else {
            let old_radix = self.radix;
            self.set_radix(radix);

            let res = self.next_float::<T>();
            self.set_radix(old_radix);

            res
        }
    }
}


/// Private helper functions for Scanner
impl<'a> Scanner<'a> {
    /// When we read `Scanner.next()`, we must first skip over any strings
    /// in the delimiting language before we begin reading the target text.
    fn consume_leading_delims(&mut self) {
        loop {
            let length = {
                if let Ok(buf) = self.stream.fill_buf() {
                    if let Ok(text) = str::from_utf8(buf) {
                        if let Some(found) = self.delim.find(text) {
                            if found.start() > 0 {
                                return;
                            }

                            found.end()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            };

            if length == 0 {
                return;
            } else {
                self.stream.consume(length);
            }
        }
    }
}
