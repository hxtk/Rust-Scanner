// Copyright (c) Peter Sanders. All rights reserved.
// Date: 2018-02-04
extern crate buf_redux;
extern crate num;
extern crate regex;

use std::io::Read;
use std::io::BufRead;
use std::marker::Sized;
use std::str;

use buf_redux::BufReader;
use regex::Regex; // For regex "delim"
use num::Integer;
use num::Float;

#[cfg(test)]
mod tests;

const DEFAULT_BUF_SIZE: usize = 1024 * 8;

/// Rust implementation of java.util.Scanner
pub struct Scanner<R: Read + Sized> {
    stream: BufReader<R>, // Underlying stream object we are handling.
    delim: Regex,  // Delimiter used to specify word boundaries.
    radix: u32,  // Base in which we parse numeric types.

    // See `impl BufRead for Scanner` block for details.
    // TODO(hxtk): Implement BufRead. Pending Issue #5.
}

/// Implements the meta-methods of Scanner that affect how the data stream
/// is processed, e.g., delimiter, parsing radix, etc.
impl<R: Read + Sized> Scanner<R> {
    /// Sets the delimiter to be some pre-compiled regex and return it
    /// for behavioral consistency.
    pub fn set_delim(&mut self, delim: Regex) -> &Regex {
        self.delim = delim;

        &self.delim
    }

    /// Sets the delimiter to be a string literal. The resulting delimiting
    /// expression is guaranteed to only interpret the literal passed in,
    /// i.e., this method **cannot** be used to simultaneously compile and set
    /// an arbitrary regular expression.
    ///
    /// We return the compiled delimiting expression.
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
    ///
    /// We return the postcondition value of the radix, which is the input
    /// if the input is within the valid range or the precondition value
    /// otherwise.
    pub fn set_radix(&mut self, radix: u32) -> u32 {
        if 1 < radix && radix <= 36 {
            self.radix = radix;
        }
        self.radix
    }

    /// Retrieve the radix on which we perform numeric parsing.
    pub fn get_radix(&self) -> u32 {
        self.radix
    }
}

/// Implements the methods of Scanner that affect the underlying data stream
impl<R: Read + Sized> Scanner<R> {
    /// Creates a new instance of Scanner on some object implementing `Read`
    pub fn new(stream: R) -> Scanner<R> {
        Scanner {
            stream: BufReader::new(stream),
            // We can safely unwrap this regex because it is hard-coded.
            delim: Regex::new(r"\s+").unwrap(),
            radix: 10,
        }
    }

    /// Creates a new instance of Scanner using a BufReader with a specified
    /// buffer size.
    ///
    /// This instantiator allows the user to specify the capacity of the buffer.
    /// Its primary use-case is unit testing this module, i.e., it would be
    /// cumbersome to write 64KB test strings so one might specify a
    /// capacity of only a few bytes in order to test what happens at the
    pub fn with_capacity(size: usize, stream: R) -> Scanner<R> {
        Scanner {
            stream: BufReader::with_capacity(size, stream),
            // We can safely unwrap this regex because it is hard-coded.
            delim: Regex::new(r"\s+").unwrap(),
            radix: 10,
        }
    }

    /// Returns `Some(String)` containing the next string if there is one.
    /// Otherwise returns `None`.
    ///
    /// We first consume all leading `delim`s that fit within the buffer of the
    /// underlying `BufRead`, then attempt to read everything until
    /// (but excluding) the next `delim` which is entirely contained within a
    /// single buffer. We guarantee this will behave as expected if the longest
    /// single precendent delimiter is no larger than the size of the buffer.
    ///
    /// Otherwise it will fail.
    pub fn next(&mut self) -> Option<String> {
        let offset = {
            self.leading_delims_offset()
        };
        self.stream.consume(offset);

        let delim_idx;
        let mut res = String::new();
        let mut last_length = 0;

        loop {
                
            let delta = {
                if let Ok(_size) = self.stream.read_into_buf() {
                    let buf = self.stream.get_buf();
                    // If the buffer is not a valid utf-8 string, we exit the
                    // method with `None` result.
                    if str::from_utf8(buf).is_err() {
                        return None;
                    }
                    
                    // The check above guarantees `unwrap` will succeed.
                    res = String::from(str::from_utf8(buf).unwrap());

                    let old_len = last_length;
                    last_length = buf.len();

                    buf.len() - old_len
                } else {
                    0
                }
            };
            
            if delta == 0 {
                delim_idx = res.len();
                break;
            }

            // If a delimiter is found within the result string, we stop reading
            // and mark the location. Everything up to here should be consumed.
            if let Some(found) = self.delim.find(res.as_str()) {
                delim_idx = found.start();
                break;
            } else {
                self.stream.grow(DEFAULT_BUF_SIZE);
            }
        }
        self.stream.consume(delim_idx);

        res.truncate(delim_idx);
        res.shrink_to_fit();
        
        Some(res)
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
impl<R: Read + Sized> Scanner<R> {
    /// When we read `Scanner.next()`, we must first skip over any strings
    /// in the delimiting language before we begin reading the target text.
    fn leading_delims_offset(&mut self) -> usize {
        let mut res: usize = 0;

        // We move `make_room` to the front because we are no longer consuming
        // so multiple calls to it is just needless overhead
        self.stream.make_room();

        loop {
            let length = {
                if let Ok(buf) = self.stream.fill_buf() {
                    // Note that since we are no longer consuming delims as
                    // we find them, we must now slice into the buffer to
                    // skip delims we've already encountered.
                    if let Ok(text) = str::from_utf8(&buf[res..]) {
                        if let Some(found) = self.delim.find(text) {
                            if found.start() > 0 {
                                return res;
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
                return res;
            } else {
                res += length;
            }
        }
    }
/*
    /// When we read `Scanner.next()` and `Scanner.has_next()`, we are doing
    /// the same basic work, which has been exported here to avoid repetition.
    ///
    /// We require that all leading delimiters have already been dealt with
*/
}
