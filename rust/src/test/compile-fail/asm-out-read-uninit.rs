// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-s390x
// ignore-emscripten
// ignore-powerpc
// ignore-powerpc64
// ignore-powerpc64le
// ignore-sparc
// ignore-sparc64
// ignore-mips
// ignore-mips64

// revisions: ast mir
//[mir]compile-flags: -Z borrowck=mir

#![feature(asm)]

fn foo(x: isize) { println!("{}", x); }

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "arm",
          target_arch = "aarch64"))]
pub fn main() {
    let x: isize;
    unsafe {
        asm!("mov $1, $0" : "=r"(x) : "r"(x));
        //[ast]~^ ERROR use of possibly uninitialized variable: `x`
        //[mir]~^^ ERROR use of possibly uninitialized variable: `x`
    }
    foo(x);
}

#[cfg(not(any(target_arch = "x86",
              target_arch = "x86_64",
              target_arch = "arm",
              target_arch = "aarch64")))]
pub fn main() {}
