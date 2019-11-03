// run-pass
// ignore-wasm32-bare compiled with panic=abort by default

// This test checks panic emitted from `mem::{uninitialized,zeroed}`.

#![feature(never_type)]
#![allow(deprecated, invalid_value)]

use std::{mem, panic};
use std::ptr::NonNull;

#[allow(dead_code)]
struct Foo {
    x: u8,
    y: !,
}

enum Bar {}

fn test_panic_msg<T>(op: impl (FnOnce() -> T) + panic::UnwindSafe, msg: &str) {
    let err = panic::catch_unwind(op).err();
    assert_eq!(
        err.as_ref().and_then(|a| a.downcast_ref::<String>()).map(|s| &**s),
        Some(msg)
    );
}

fn main() {
    unsafe {
        // Uninitialized types
        test_panic_msg(
            || mem::uninitialized::<!>(),
            "attempted to instantiate uninhabited type `!`"
        );
        test_panic_msg(
            || mem::zeroed::<!>(),
            "attempted to instantiate uninhabited type `!`"
        );
        test_panic_msg(
            || mem::MaybeUninit::<!>::uninit().assume_init(),
            "attempted to instantiate uninhabited type `!`"
        );

        test_panic_msg(
            || mem::uninitialized::<Foo>(),
            "attempted to instantiate uninhabited type `Foo`"
        );
        test_panic_msg(
            || mem::zeroed::<Foo>(),
            "attempted to instantiate uninhabited type `Foo`"
        );
        test_panic_msg(
            || mem::MaybeUninit::<Foo>::uninit().assume_init(),
            "attempted to instantiate uninhabited type `Foo`"
        );

        test_panic_msg(
            || mem::uninitialized::<Bar>(),
            "attempted to instantiate uninhabited type `Bar`"
        );
        test_panic_msg(
            || mem::zeroed::<Bar>(),
            "attempted to instantiate uninhabited type `Bar`"
        );
        test_panic_msg(
            || mem::MaybeUninit::<Bar>::uninit().assume_init(),
            "attempted to instantiate uninhabited type `Bar`"
        );

        // Types that do not like zero-initialziation
        test_panic_msg(
            || mem::uninitialized::<fn()>(),
            "attempted to zero-initialize non-zero type `fn()`"
        );
        test_panic_msg(
            || mem::zeroed::<fn()>(),
            "attempted to zero-initialize non-zero type `fn()`"
        );

        test_panic_msg(
            || mem::uninitialized::<*const dyn Send>(),
            "attempted to zero-initialize non-zero type `*const dyn std::marker::Send`"
        );
        test_panic_msg(
            || mem::zeroed::<*const dyn Send>(),
            "attempted to zero-initialize non-zero type `*const dyn std::marker::Send`"
        );

        test_panic_msg(
            || mem::uninitialized::<(NonNull<u32>, u32, u32)>(),
            "attempted to zero-initialize non-zero type `(std::ptr::NonNull<u32>, u32, u32)`"
        );
        test_panic_msg(
            || mem::zeroed::<(NonNull<u32>, u32, u32)>(),
            "attempted to zero-initialize non-zero type `(std::ptr::NonNull<u32>, u32, u32)`"
        );
    }
}
