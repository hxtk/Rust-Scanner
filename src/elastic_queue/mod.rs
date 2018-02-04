// Copyright (c) Peter Sanders. All rights reserved.
// Date: 2018-02-03

use std::vec::Vec;

#[cfg(test)]
mod tests;

const DEFAULT_BUF_SIZE: usize = 1024*64;  // The default used by `BufReader`.

/// In order to implement BufRead with non-destructive lookahead further than
/// the size of the buffer, we require a data structure with three qualities:
///
/// - It shall support slicing to be compatible with `BufRead.fill_buf()`
/// - It shall "stretch", i.e., grow in capacity on a temporary basis, to
///   support further lookahead without destroying its contents.
/// - It shall have the Queue property that input corresponds to output.
///
/// ## Elasticity
/// 
/// ### Fixed Minimum Capacity
///
/// The buffer cannot be allowed to shrink arbitrarily small or else we have
/// defeated the purpose of buffered reads, especially when reading files or
/// network streams.
///
/// > Note: This does **not** mean the buffer must always be filled to capacity.
///
/// ### Stretch
///
/// The buffer must be able to accommodate more data than its capacity when
/// requested. By principle of least astonishment, this should require a
/// special function and the default behavior should be to respect the capacity.
///
/// ### Contraction
///
/// To avoid memory bloat, after the buffer has been "stretched", future
/// destructive calls should shrink its capacity until it has returned to its
/// minimum capacity.
///
/// ## Sliceability
///
/// `trait BufRead` requires that we return a `slice` of the filled buffer when
/// a call is made to `fill_buf()`. We would like to do this in O(1) using the
/// normal `slice` operation since this will represent nearly all read
/// operations, and the tradeoff is against insertion, which will be O(n) as a
/// result of this decision but will only occur when the buffer is empty or
/// stretch is requested.
///
/// ## Queue
///
/// We want to extract values in the same order that they were read.
pub struct ElasticQueue<T> {
    buf: Vec<T>,  // Resizeable, sliceable list to serve as buffer

    cap: usize,  // The persistent capacity of the buffer
                 // Note this may differ from `buf.capacity()` in cases
                 // where we have stretched the buffer.

    read_pos: usize,  // The position in the buffer of the first entry.
    write_pos: usize, // The position in the buffer after the last entry.
}

/// # Instantiators for ElasticQueue
impl<T> ElasticQueue<T> {

    /// This is the default instantiator. It creates a buffer with a default
    /// capacity of 64K, keeping parity with the default size of `BufReader`'s
    /// 64KB capacity when the type is a one-byte type, e.g. `u8`.
    ///
    /// For primitive or reference types in applications with loose memory
    /// constraints, one should usually choose this function.
    pub fn new() -> ElasticQueue<T> {
        Self {
            buf: Vec::with_capacity(DEFAULT_BUF_SIZE),
            cap: DEFAULT_BUF_SIZE,
            pos: 0
        }
    }

    /// This instantiator allows the user to specify the capacity. Its
    /// primary use-case is unit testing this module, i.e., it would be
    /// cumbersome to write 64KB test strings so one might specify a
    /// capacity of only a few bytes.
    ///
    /// For non-primitive, non-reference types, it is recommended that
    /// one use this function and choose a smaller capacity.
    pub fn with_capacity(capacity: usize) -> ElasticQueue<T> {
        Self {
            buf: Vec::with_capacity(capacity),
            cap: capacity,
            pos: 0
        }
    }
}

impl<T> ElasticQueue<T> {

    pub fn enqueue(item: T) {
        
    }
}
