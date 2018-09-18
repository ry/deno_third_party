// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Regression test for #51008 -- the anonymous lifetime in `&i32` was
// being incorrectly considered part of the "elided lifetimes" from
// the impl.
//
// run-pass

#![feature(rust_2018_preview)]

trait A {

}

impl<F> A for F where F: PartialEq<fn(&i32)> { }

fn main() {}
