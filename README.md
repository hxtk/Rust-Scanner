# Scanner

A port of Java's `java.util.Scanner` interface to Rust.

## Installing / Getting started

This project currently does not build. This section will be updated on the
first release.

## Developing

Code should be styled according to `rustfmt`. There is currently no behavior
in our roadmap that should require the use of an `unsafe` block.

All `unwrap()`s should either immediately follow a check that they are safe or
include a comment explaining why they are guaranteed to be safe.

Finally, we are following test-driven development. Use the behavior of `java.util.Scanner` as the reference implementation for a test, with the exception that exceptions should never be raised in a situation where a wrapped return value, e.g., `Option`, `Result`, would be appropriate. This will present some places where we will not have parity with Java.

### Deploying / Publishing

This section will be updated after we have a working release. In short, we plan on making this available to Cargo and it should be more or less plug-and-play.

## Features

### Complete

- `Scanner.next() -> Option<String>`

- `Scanner.next_line() -> Option<String>`

- `Scanner.next_i32() -> Option<i32>`

- Support for regular language delimiters

### Road Map

1. Support for `next[primitive]()` methods. We have already implemented `next_i32`, but this API is likely to be subject to breaking changes in the future: it is easy to make this function generic over, e.g., integral types, floating point types, etc.

## Contributing

In general, feel free to work on any feature that is found in `java.util.Scanner` that we have not implemented here. The repository owner(s) reserve the right to reject pull requests. Here are some tips to make sure your pull request is accepted:

- Comment on the issue for the feature you are writing. If there isn't an issue, make one.

- Get approval from one of the main contributors.

- Include unit tests for your feature. Where there is pairity with `java.util.Scanner`, ensure those tests match its behavior (see exception caveat under "Developing").
