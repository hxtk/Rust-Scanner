# Scanner [![Build Status](https://travis-ci.org/hxtk/Rust-Scanner.png?branch=master)](https://travis-ci.org/hxtk/Rust-Scanner) [![crates.io](https://img.shields.io/crates/v/file_scanner.svg)](https://crates.io/crates/file_scanner) [![codecov](https://codecov.io/gh/hxtk/Rust-Scanner/branch/master/graph/badge.svg)](https://codecov.io/gh/hxtk/Rust-Scanner)


A port of Java's `java.util.Scanner` interface to Rust.

## Installing / Getting started

This project is available on [crates.io](https://crates.io/crates/file_scanner).

For the most recent version, always check this repository.

## Developing

Code should be styled according to `rustfmt`. There is currently no behavior
in our roadmap that should require the use of an `unsafe` block.

All `unwrap()`s should either immediately follow a check that they are safe or
include a comment explaining why they are guaranteed to be safe.

We are following test-driven development. Use the behavior of `java.util.Scanner` as the reference implementation for a test, with the exception that exceptions should never be raised in a situation where a wrapped return value, e.g., `Option`, `Result`, would be appropriate. This will present some places where we will not have parity with Java.

Finally, have fun. There is no need to be overly strict about 1:1 parity with Java, but it does offer a good pattern for complex IO without variadic functions or operator overloading that one finds in C and C++'s approaches. We are attempting to keep parity so that developers familiar with the idioms of Java's Scanner will have a fast learning curve, but this is also an opportunity to air grievances with Java's Scanner, so long as you can justify them.

### Deploying / Publishing

Simply add `file_scanner = "0.2.0"` to your `[dependencies]`.

Note changes from previous version: We now take an immutable object implementing `Read` instead of a mutable reference to an object implementing `BufRead`. See example code below.

```rust
extern crate file_scanner;
use file_scanner::Scanner;

//snip

let file = File::open(...)?;
let mut s = Scanner::new(file);

let int = s.next_int().unwrap();
let bin = s.next_int_radix(2).unwrap();
let real = s.next_float().unwrap();
let hex_real = s.next_float_radix(16).unwrap();

let word = s.next().unwrap();
let line = s.next_line().unwrap();

s.set_delim_str("[ foo ]");  // words will now be delimited by "[ foo ]"

// words are delimited by whitespace (this is the default behavior)
s.set_delim(Regex::new(r"\s+").unwrap());

s.set_radix(2);  // future calls to next_int or next_float will use binary
s.set_radix(16);  // hexadecimal
s.set_radix(36);  // alphanumeric
// or anything in between
```

For full documentation, see https://hxtk.github.io/Rust-Scanner/file_scanner/

Note we are currently tracking a bug where precedent delimiters larger than the buffer are undetectable. See [Issue #4](https://github.com/hxtk/Rust-Scanner/issues/4) for details.

## Features

### Complete

- `Scanner.next() -> Option<String>`

- `Scanner.next_line() -> Option<String>`

- `Scanner.next_int<T: Integer>() -> Option<T>`

- Support for regular language delimiters*

- `Scanner.next_float<T: Float>() -> Option<T>`

- Arbitrary radix integer parsing

### Road Map

- `Scanner.has_next*`

## Contributing

In general, feel free to work on any feature that is found in `java.util.Scanner` that we have not implemented here. The repository owner(s) reserve the right to reject pull requests. Here are some tips to make sure your pull request is accepted:

- Comment on the issue for the feature you are writing. If there isn't an issue, make one.

- Get approval from one of the main contributors.

- Include unit tests for your feature. Where there is pairity with `java.util.Scanner`, ensure those tests match its behavior (see exception caveat under "Developing").

One of the best ways you can help if you don't have time to implement a feature is to nitpick (politely). The lead developer is still a student of both Rust and software engineering in general.
