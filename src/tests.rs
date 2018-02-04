// Copyright (c) Peter Sanders. All rights reserved.
// Date: 2018-02-03
//
// Unit tests for Rust implementation of Scanner.
extern crate buf_redux;

use super::*;

use buf_redux::BufReader;

#[test]
fn next_works_once_when_good_input() {
    let string: &[u8] = b"hello";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next(), Some(String::from("hello")));
}

#[test]
fn next_breaks_at_delim() {
    let string: &[u8] = b"hello, world";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next(), Some(String::from("hello,")));
}

#[test]
fn next_skips_leading_delims() {
    let string: &[u8] = b"hello,  world";
    let mut test = Scanner::new(BufReader::new(string));
    test.next();

    assert_eq!(test.next(), Some(String::from("world")));
}

/// When this test was written, the first delimiter character after
/// the string read by `Scanner.next()` would be consumed, which affected
/// the result of the next data operation if that operation used a different
/// delimiter.
#[test]
fn next_preserves_trailing_delim() {
    let string: &[u8] = b"hello,  world";
    let mut test = Scanner::new(BufReader::new(string));

    test.next();
    assert_eq!(test.next_line(), Some(String::from("  world")));
}

#[test]
fn next_handles_line_wrap() {
    let string: &[u8] = b"hello\nworld";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next(), Some(String::from("hello")));
}

#[test]
fn next_line_reads_whole_line() {
    let string: &[u8] = b"hello,  world\ngoodbye, world";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next_line(), Some(String::from("hello,  world")));
}

#[test]
fn next_line_reads_last_line() {
    let string: &[u8] = b"foo bar baz";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next_line(), Some(String::from("foo bar baz")));
}

#[test]
fn next_works_after_next_line() {
    let string: &[u8] = b"hello,  world\ngoodbye, world";
    let mut test = Scanner::new(BufReader::new(string));
    test.next_line();

    assert_eq!(test.next(), Some(String::from("goodbye,")));
}

#[test]
fn next_int_handles_commas() {
    let string: &[u8] = b"2,147,483,647";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next_int::<i32>(), Some(2147483647));
}

#[test]
fn next_int_none_on_positive_overflow() {
    let string: &[u8] = b"2147483648";
    let mut test = Scanner::new(BufReader::new(string));

    let res = test.next_int::<i32>();
    assert_eq!(res, None);
}

#[test]
fn next_i32_none_on_negative_overflow() {
    let string: &[u8] = b"-2147483649";
    let mut test = Scanner::new(BufReader::new(string));

    let res = test.next_int::<i32>();
    assert_eq!(res, None);
}

#[test]
fn arbitrary_delim() {
    let string: &[u8] = b"foohello, worldfoo";
    let mut test = Scanner::new(BufReader::new(string));
    test.set_delim(Regex::new(r"foo").unwrap());

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "hello, world");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_float() {
    let string: &[u8] = b"2.5";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.next_float::<f64>(), Some(2.5));
}

#[test]
fn next_int_custom_radix() {
    let string: &[u8] = b"11010";
    let mut test = Scanner::new(BufReader::new(string));

    // invalid radix should return None and not consume `Scanner.next()`
    assert_eq!(test.next_int_radix::<i32>(1), None);

    // 2 is a valid radix.
    assert_eq!(test.next_int_radix(2), Some(26));
}

#[test]
fn next_float_base_2() {
    let string: &[u8] = b"11010.1";
    let mut test = Scanner::new(BufReader::new(string));

    // invalid radix should return None and not consume `Scanner.next()`
    assert_eq!(test.next_float_radix::<f64>(1), None);

    // 2 is a valid radix.
    assert_eq!(test.next_float_radix(2), Some(26.5));
}

#[test]
fn str_delim_escapes_regexes() {
    let string: &[u8] = b"foo[a-z]+bar";
    let mut test = Scanner::new(BufReader::new(string));
    test.set_delim_str("[a-z]+");

    test.next();
    assert_eq!(test.next(), Some(String::from("bar")));
}

#[test]
fn radix_between_2_36() {
    let string: &[u8] = b"";
    let mut test = Scanner::new(BufReader::new(string));

    assert_eq!(test.get_radix(), 10);
    test.set_radix(1);
    assert_eq!(test.get_radix(), 10);
    test.set_radix(37);
    assert_eq!(test.get_radix(), 10);
    test.set_radix(36);
    assert_eq!(test.get_radix(), 36);
}

/// This test will fail if we cannot read past the length of the buffer.
/// The buffer size is four characters, so it will read "hell". If we do
/// not continue past the buffer, then it is interpreted as if we have
/// reached EOF. This affects searching for the terminating delimiter.
#[test]
fn buffer_ends_before_delim() {
    let string: &[u8] = b"hello world";
    let mut test = Scanner::new(BufReader::with_capacity(4, string));

    assert_eq!(test.next(), Some(String::from("hello")));
}


/// This test will fail if we do not solve the above problem in a way that
/// preserves the tail of the original buffer, because in this test case the
/// terminating delimiter begins within the first buffer size and
/// ends within the second.
#[test]
fn buffer_ends_within_end_delim() {
    let string: &[u8] = b"foo  bar";
    let mut test = Scanner::new(BufReader::with_capacity(4, string));
    test.set_delim_str("  ");

    assert_eq!(test.next(), Some(String::from("foo")));
}

/// This test will fail if we cannot detect partial matches of the delimiter
/// when skipping over prefixed delimiters. Because the buffer size is 4, it
/// will read "aaaa", which is not in the language of /a+b/, however the
/// automaton is not in a dead state either: reading a "b" would put us in
/// an accepting state, thus we must read more input to know if the regex is
/// satisfied. Reading an additional character will result in "aaaab", which
/// is a valid delimiter in this language and should therefore be skipped.
#[test]
fn buffer_ends_within_start_delim() {
    let string: &[u8] = b"aaaabfoo";
    let mut test = Scanner::new(BufReader::with_capacity(4, string));
    test.set_delim(Regex::new(r"a+b").unwrap());

    assert_eq!(test.next(), Some(String::from("foo")));
}

/// This test will fail if the above problem's solution does not preserve
/// greedy operators: If the whole buffer is a delimiter, but a greedy
/// operator means it is also a prefix to a delimiter, it should not be
/// consumed until we determine whether greedy operators would extend the
/// delimiter further into the next buffer.
///
/// Note that this only applies to precedent delimiters because it does not
/// matter where a terminating delimiter ends so long as we may accurately
/// pinpoint its beginning.
#[test]
fn buffer_boundary_preserves_greed() {
    let string: &[u8] = b"aaabbfoo";
    let mut test = Scanner::new(BufReader::with_capacity(4, string));
    test.set_delim(Regex::new(r"a[ab]*b").unwrap());

    // If this test fails, we expect it to produce "bfoo" instead of "foo".
    assert_eq!(test.next(), Some(String::from("foo")));
}
