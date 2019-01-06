// ignore-windows
// aux-build:cfg-target-thread-local.rs

#![feature(thread_local)]

extern crate cfg_target_thread_local;

extern {
    #[cfg_attr(target_thread_local, thread_local)]
    //~^ `cfg(target_thread_local)` is experimental and subject to change (see issue #29594)

    static FOO: u32;
}

fn main() {
    assert_eq!(FOO, 3);
}
